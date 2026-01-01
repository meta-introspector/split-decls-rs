# AST Trace: ../rust/library/std/src/os/fd/owned.rs

Generated 50 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
//! Owned and borrowed Unix-like file descriptors.

#![stable(feature = "io_safety", since = "1.63.0")]
#![deny(unsafe_op_in_unsafe_fn)]

use super::raw::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=13

```rust
#[cfg(not(target_os = "trusty"))]
use crate::fs;
use crate::marker::PhantomData;
use crate::mem::ManuallyDrop;
#[cfg(not(any(
    target_arch = "wasm32",
    target_env = "sgx",
    target_os = "hermit",
    target_os = "trusty"
)))]
use crate::sys::cvt;
#[cfg(not(target_os = "trusty"))]
use crate::sys_common::{AsInner, FromInner, IntoInner};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{fmt, io};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=BorrowedFd | COMPLEXITY=6 | LINES=30

```rust
type ValidRawFd = core::num::niche_types::NotAllOnes<RawFd>;

/// A borrowed file descriptor.
///
/// This has a lifetime parameter to tie it to the lifetime of something that owns the file
/// descriptor. For the duration of that lifetime, it is guaranteed that nobody will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as an argument, it is not captured or consumed, and it never has the
/// value `-1`.
///
/// This type does not have a [`ToOwned`][crate::borrow::ToOwned]
/// implementation. Calling `.to_owned()` on a variable of this type will call
/// it on `&BorrowedFd` and use `Clone::clone()` like `ToOwned` does for all
/// types implementing `Clone`. The result will be descriptor borrowed under
/// the same lifetime.
///
/// To obtain an [`OwnedFd`], you can use [`BorrowedFd::try_clone_to_owned`]
/// instead, but this is not supported on all platforms.
#[derive(Copy, Clone)]
#[repr(transparent)]
#[rustc_nonnull_optimization_guaranteed]
#[stable(feature = "io_safety", since = "1.63.0")]
pub struct BorrowedFd<'fd> {
    fd: ValidRawFd,
    _phantom: PhantomData<&'fd OwnedFd>,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=OwnedFd | COMPLEXITY=3 | LINES=18

```rust
/// An owned file descriptor.
///
/// This closes the file descriptor on drop. It is guaranteed that nobody else will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as a consumed argument or returned as an owned value, and it never
/// has the value `-1`.
///
/// You can use [`AsFd::as_fd`] to obtain a [`BorrowedFd`].
#[repr(transparent)]
#[rustc_nonnull_optimization_guaranteed]
#[stable(feature = "io_safety", since = "1.63.0")]
pub struct OwnedFd {
    fd: ValidRawFd,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=16

```rust
impl BorrowedFd<'_> {
    /// Returns a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    #[inline]
    #[track_caller]
    #[rustc_const_stable(feature = "io_safety", since = "1.63.0")]
    #[stable(feature = "io_safety", since = "1.63.0")]
    pub const unsafe fn borrow_raw(fd: RawFd) -> Self {
        Self { fd: ValidRawFd::new(fd).expect("fd != -1"), _phantom: PhantomData }
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=try_clone | COMPLEXITY=3 | LINES=9

```rust
impl OwnedFd {
    /// Creates a new `OwnedFd` instance that shares the same underlying file
    /// description as the existing `OwnedFd` instance.
    #[stable(feature = "io_safety", since = "1.63.0")]
    pub fn try_clone(&self) -> crate::io::Result<Self> {
        self.as_fd().try_clone_to_owned()
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=try_clone_to_owned | COMPLEXITY=21 | LINES=33

```rust
impl BorrowedFd<'_> {
    /// Creates a new `OwnedFd` instance that shares the same underlying file
    /// description as the existing `BorrowedFd` instance.
    #[cfg(not(any(target_arch = "wasm32", target_os = "hermit", target_os = "trusty")))]
    #[stable(feature = "io_safety", since = "1.63.0")]
    pub fn try_clone_to_owned(&self) -> crate::io::Result<OwnedFd> {
        // We want to atomically duplicate this file descriptor and set the
        // CLOEXEC flag, and currently that's done via F_DUPFD_CLOEXEC. This
        // is a POSIX flag that was added to Linux in 2.6.24.
        #[cfg(not(any(target_os = "espidf", target_os = "vita")))]
        let cmd = libc::F_DUPFD_CLOEXEC;

        // For ESP-IDF, F_DUPFD is used instead, because the CLOEXEC semantics
        // will never be supported, as this is a bare metal framework with
        // no capabilities for multi-process execution. While F_DUPFD is also
        // not supported yet, it might be (currently it returns ENOSYS).
        #[cfg(any(target_os = "espidf", target_os = "vita"))]
        let cmd = libc::F_DUPFD;

        // Avoid using file descriptors below 3 as they are used for stdio
        let fd = cvt(unsafe { libc::fcntl(self.as_raw_fd(), cmd, 3) })?;
        Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    }

    /// Creates a new `OwnedFd` instance that shares the same underlying file
    /// description as the existing `BorrowedFd` instance.
    #[cfg(any(target_arch = "wasm32", target_os = "hermit", target_os = "trusty"))]
    #[stable(feature = "io_safety", since = "1.63.0")]
    pub fn try_clone_to_owned(&self) -> crate::io::Result<OwnedFd> {
        Err(crate::io::Error::UNSUPPORTED_PLATFORM)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=as_raw_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_inner()
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=as_raw_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_inner()
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=into_raw_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        ManuallyDrop::new(self).fd.as_inner()
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=13 | LINES=17

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl FromRawFd for OwnedFd {
    /// Constructs a new instance of `Self` from the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must be open and suitable for assuming
    /// [ownership][io-safety]. The resource must not require any cleanup other than `close`.
    ///
    /// [io-safety]: io#io-safety
    #[inline]
    #[track_caller]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self { fd: ValidRawFd::new(fd).expect("fd != -1") }
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=14 | LINES=29

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            // Note that errors are ignored when closing a file descriptor. According to POSIX 2024,
            // we can and indeed should retry `close` on `EINTR`
            // (https://pubs.opengroup.org/onlinepubs/9799919799.2024edition/functions/close.html),
            // but it is not clear yet how well widely-used implementations are conforming with this
            // mandate since older versions of POSIX left the state of the FD after an `EINTR`
            // unspecified. Ignoring errors is "fine" because some of the major Unices (in
            // particular, Linux) do make sure to always close the FD, even when `close()` is
            // interrupted, and the scenario is rare to begin with. If we retried on a
            // not-POSIX-compliant implementation, the consequences could be really bad since we may
            // close the wrong FD. Helpful link to an epic discussion by POSIX workgroup that led to
            // the latest POSIX wording: http://austingroupbugs.net/view.php?id=529
            #[cfg(not(target_os = "hermit"))]
            {
                #[cfg(unix)]
                crate::sys::fs::debug_assert_fd_is_open(self.fd.as_inner());

                let _ = libc::close(self.fd.as_inner());
            }
            #[cfg(target_os = "hermit")]
            let _ = hermit_abi::close(self.fd.as_inner());
        }
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl fmt::Debug for BorrowedFd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedFd").field("fd", &self.fd).finish()
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl fmt::Debug for OwnedFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedFd").field("fd", &self.fd).finish()
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=is_terminal | COMPLEXITY=15 | LINES=15

```rust
macro_rules! impl_is_terminal {
    ($($t:ty),*$(,)?) => {$(
        #[unstable(feature = "sealed", issue = "none")]
        impl crate::sealed::Sealed for $t {}

        #[stable(feature = "is_terminal", since = "1.70.0")]
        impl crate::io::IsTerminal for $t {
            #[inline]
            fn is_terminal(&self) -> bool {
                crate::sys::io::is_terminal(self)
            }
        }
    )*}
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=4 | LINES=28

```rust
impl_is_terminal!(BorrowedFd<'_>, OwnedFd);

/// A trait to borrow the file descriptor from an underlying object.
///
/// This is only available on unix platforms and must be imported in order to
/// call the method. Windows platforms have a corresponding `AsHandle` and
/// `AsSocket` set of traits.
#[stable(feature = "io_safety", since = "1.63.0")]
pub trait AsFd {
    /// Borrows the file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// # use std::os::fd::{AsFd, BorrowedFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// # #[cfg(any(unix, target_os = "wasi"))]
    /// let borrowed_fd: BorrowedFd<'_> = f.as_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    #[stable(feature = "io_safety", since = "1.63.0")]
    fn as_fd(&self) -> BorrowedFd<'_>;
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: AsFd + ?Sized> AsFd for &T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl<T: AsFd + ?Sized> AsFd for &mut T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl AsFd for BorrowedFd<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        *self
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=10 | LINES=11

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl AsFd for OwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // Safety: `OwnedFd` and `BorrowedFd` have the same validity
        // invariants, and the `BorrowedFd` is bounded by the lifetime
        // of `&self`.
        unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) }
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl AsFd for fs::File {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.as_inner().as_fd()
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<fs::File> for OwnedFd {
    /// Takes ownership of a [`File`](fs::File)'s underlying file descriptor.
    #[inline]
    fn from(file: fs::File) -> OwnedFd {
        file.into_inner().into_inner().into_inner()
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<OwnedFd> for fs::File {
    /// Returns a [`File`](fs::File) that takes ownership of the given
    /// file descriptor.
    #[inline]
    fn from(owned_fd: OwnedFd) -> Self {
        Self::from_inner(FromInner::from_inner(FromInner::from_inner(owned_fd)))
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl AsFd for crate::net::TcpStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.as_inner().socket().as_fd()
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<crate::net::TcpStream> for OwnedFd {
    /// Takes ownership of a [`TcpStream`](crate::net::TcpStream)'s socket file descriptor.
    #[inline]
    fn from(tcp_stream: crate::net::TcpStream) -> OwnedFd {
        tcp_stream.into_inner().into_socket().into_inner().into_inner().into()
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<OwnedFd> for crate::net::TcpStream {
    #[inline]
    fn from(owned_fd: OwnedFd) -> Self {
        Self::from_inner(FromInner::from_inner(FromInner::from_inner(FromInner::from_inner(
            owned_fd,
        ))))
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl AsFd for crate::net::TcpListener {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.as_inner().socket().as_fd()
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<crate::net::TcpListener> for OwnedFd {
    /// Takes ownership of a [`TcpListener`](crate::net::TcpListener)'s socket file descriptor.
    #[inline]
    fn from(tcp_listener: crate::net::TcpListener) -> OwnedFd {
        tcp_listener.into_inner().into_socket().into_inner().into_inner().into()
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<OwnedFd> for crate::net::TcpListener {
    #[inline]
    fn from(owned_fd: OwnedFd) -> Self {
        Self::from_inner(FromInner::from_inner(FromInner::from_inner(FromInner::from_inner(
            owned_fd,
        ))))
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=9

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl AsFd for crate::net::UdpSocket {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.as_inner().socket().as_fd()
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=10

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<crate::net::UdpSocket> for OwnedFd {
    /// Takes ownership of a [`UdpSocket`](crate::net::UdpSocket)'s file descriptor.
    #[inline]
    fn from(udp_socket: crate::net::UdpSocket) -> OwnedFd {
        udp_socket.into_inner().into_socket().into_inner().into_inner().into()
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=11

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
#[cfg(not(target_os = "trusty"))]
impl From<OwnedFd> for crate::net::UdpSocket {
    #[inline]
    fn from(owned_fd: OwnedFd) -> Self {
        Self::from_inner(FromInner::from_inner(FromInner::from_inner(FromInner::from_inner(
            owned_fd,
        ))))
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=9 | LINES=16

```rust
#[stable(feature = "asfd_ptrs", since = "1.64.0")]
/// This impl allows implementing traits that require `AsFd` on Arc.
/// ```
/// # #[cfg(any(unix, target_os = "wasi"))] mod group_cfg {
/// # #[cfg(target_os = "wasi")]
/// # use std::os::wasi::io::AsFd;
/// # #[cfg(unix)]
/// # use std::os::unix::io::AsFd;
/// use std::net::UdpSocket;
/// use std::sync::Arc;
///
/// trait MyTrait: AsFd {}
/// impl MyTrait for Arc<UdpSocket> {}
/// impl MyTrait for Box<UdpSocket> {}
/// # }
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=7

```rust
/// ```
impl<T: AsFd + ?Sized> AsFd for crate::sync::Arc<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "asfd_rc", since = "1.69.0")]
impl<T: AsFd + ?Sized> AsFd for crate::rc::Rc<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[unstable(feature = "unique_rc_arc", issue = "112566")]
impl<T: AsFd + ?Sized> AsFd for crate::rc::UniqueRc<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "asfd_ptrs", since = "1.64.0")]
impl<T: AsFd + ?Sized> AsFd for Box<T> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        (**self).as_fd()
    }
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=10 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl AsFd for io::Stdin {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(0) }
    }
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=10 | LINES=9

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl<'a> AsFd for io::StdinLock<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // SAFETY: user code should not close stdin out from under the standard library
        unsafe { BorrowedFd::borrow_raw(0) }
    }
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=10 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl AsFd for io::Stdout {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(1) }
    }
}
```

## Block 42
**Metadata**: AST_ID=42 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=10 | LINES=9

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl<'a> AsFd for io::StdoutLock<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // SAFETY: user code should not close stdout out from under the standard library
        unsafe { BorrowedFd::borrow_raw(1) }
    }
}
```

## Block 43
**Metadata**: AST_ID=43 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=10 | LINES=8

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl AsFd for io::Stderr {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(2) }
    }
}
```

## Block 44
**Metadata**: AST_ID=44 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=10 | LINES=9

```rust
#[stable(feature = "io_safety", since = "1.63.0")]
impl<'a> AsFd for io::StderrLock<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        // SAFETY: user code should not close stderr out from under the standard library
        unsafe { BorrowedFd::borrow_raw(2) }
    }
}
```

## Block 45
**Metadata**: AST_ID=45 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
#[cfg(not(target_os = "trusty"))]
impl AsFd for io::PipeReader {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}
```

## Block 46
**Metadata**: AST_ID=46 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
#[cfg(not(target_os = "trusty"))]
impl From<io::PipeReader> for OwnedFd {
    fn from(pipe: io::PipeReader) -> Self {
        pipe.0.into_inner()
    }
}
```

## Block 47
**Metadata**: AST_ID=47 | TYPE=FUNCTION | NAME=as_fd | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
#[cfg(not(target_os = "trusty"))]
impl AsFd for io::PipeWriter {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}
```

## Block 48
**Metadata**: AST_ID=48 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
#[cfg(not(target_os = "trusty"))]
impl From<io::PipeWriter> for OwnedFd {
    fn from(pipe: io::PipeWriter) -> Self {
        pipe.0.into_inner()
    }
}
```

## Block 49
**Metadata**: AST_ID=49 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
#[cfg(not(target_os = "trusty"))]
impl From<OwnedFd> for io::PipeReader {
    fn from(owned_fd: OwnedFd) -> Self {
        Self(FromInner::from_inner(owned_fd))
    }
}
```

## Block 50
**Metadata**: AST_ID=50 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
#[cfg(not(target_os = "trusty"))]
impl From<OwnedFd> for io::PipeWriter {
    fn from(owned_fd: OwnedFd) -> Self {
        Self(FromInner::from_inner(owned_fd))
    }
}
```

---
*Generated by AST tracing system*
