# AST Trace: ../rust/library/std/src/os/windows/io/raw.rs

Generated 35 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
//! Windows-specific extensions to general I/O primitives.

#![stable(feature = "rust1", since = "1.0.0")]

#[cfg(doc)]
use crate::os::windows::io::{AsHandle, AsSocket};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::os::windows::io::{OwnedHandle, OwnedSocket};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::os::windows::raw;
use crate::sys_common::{AsInner, FromInner, IntoInner};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::{fs, io, net, ptr, sys};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=8 | LINES=31

```rust
/// Raw HANDLEs.
#[stable(feature = "rust1", since = "1.0.0")]
pub type RawHandle = raw::HANDLE;

/// Raw SOCKETs.
#[stable(feature = "rust1", since = "1.0.0")]
pub type RawSocket = raw::SOCKET;

/// Extracts raw handles.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRawHandle {
    /// Extracts the raw handle.
    ///
    /// This function is typically used to **borrow** an owned handle.
    /// When used in this way, this method does **not** pass ownership of the
    /// raw handle to the caller, and the handle is only guaranteed
    /// to be valid while the original object has not yet been destroyed.
    ///
    /// This function may return null, such as when called on [`Stdin`],
    /// [`Stdout`], or [`Stderr`] when the console is detached.
    ///
    /// However, borrowing is not strictly required. See [`AsHandle::as_handle`]
    /// for an API which strictly borrows a handle.
    ///
    /// [`Stdin`]: io::Stdin
    /// [`Stdout`]: io::Stdout
    /// [`Stderr`]: io::Stderr
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_raw_handle(&self) -> RawHandle;
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=18 | LINES=32

```rust
/// Constructs I/O objects from raw handles.
#[stable(feature = "from_raw_os", since = "1.1.0")]
pub trait FromRawHandle {
    /// Constructs a new I/O object from the specified raw handle.
    ///
    /// This function is typically used to **consume ownership** of the handle
    /// given, passing responsibility for closing the handle to the returned
    /// object. When used in this way, the returned object
    /// will take responsibility for closing it when the object goes out of
    /// scope.
    ///
    /// However, consuming ownership is not strictly required. Use a
    /// `From<OwnedHandle>::from` implementation for an API which strictly
    /// consumes ownership.
    ///
    /// # Safety
    ///
    /// The `handle` passed in must:
    ///   - be an [owned handle][io-safety]; in particular, it must be open.
    ///   - be a handle for a resource that may be freed via [`CloseHandle`]
    ///     (as opposed to `RegCloseKey` or other close functions).
    ///
    /// Note that the handle *may* have the value `INVALID_HANDLE_VALUE` (-1),
    /// which is sometimes a valid handle value. See [here] for the full story.
    ///
    /// [`CloseHandle`]: https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
    /// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
    /// [io-safety]: io#io-safety
    #[stable(feature = "from_raw_os", since = "1.1.0")]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self;
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=into_raw_handle | COMPLEXITY=5 | LINES=18

```rust
/// A trait to express the ability to consume an object and acquire ownership of
/// its raw `HANDLE`.
#[stable(feature = "into_raw_os", since = "1.4.0")]
pub trait IntoRawHandle {
    /// Consumes this object, returning the raw underlying handle.
    ///
    /// This function is typically used to **transfer ownership** of the underlying
    /// handle to the caller. When used in this way, callers are then the unique
    /// owners of the handle and must close it once it's no longer needed.
    ///
    /// However, transferring ownership is not strictly required. Use a
    /// `Into<OwnedHandle>::into` implementation for an API which strictly
    /// transfers ownership.
    #[must_use = "losing the raw handle may leak resources"]
    #[stable(feature = "into_raw_os", since = "1.4.0")]
    fn into_raw_handle(self) -> RawHandle;
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawHandle for fs::File {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.as_inner().as_raw_handle() as RawHandle
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=10 | LINES=7

```rust
#[stable(feature = "asraw_stdio", since = "1.21.0")]
impl AsRawHandle for io::Stdin {
    fn as_raw_handle(&self) -> RawHandle {
        stdio_handle(unsafe { sys::c::GetStdHandle(sys::c::STD_INPUT_HANDLE) as RawHandle })
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=10 | LINES=7

```rust
#[stable(feature = "asraw_stdio", since = "1.21.0")]
impl AsRawHandle for io::Stdout {
    fn as_raw_handle(&self) -> RawHandle {
        stdio_handle(unsafe { sys::c::GetStdHandle(sys::c::STD_OUTPUT_HANDLE) as RawHandle })
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=10 | LINES=7

```rust
#[stable(feature = "asraw_stdio", since = "1.21.0")]
impl AsRawHandle for io::Stderr {
    fn as_raw_handle(&self) -> RawHandle {
        stdio_handle(unsafe { sys::c::GetStdHandle(sys::c::STD_ERROR_HANDLE) as RawHandle })
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=10 | LINES=7

```rust
#[stable(feature = "asraw_stdio_locks", since = "1.35.0")]
impl<'a> AsRawHandle for io::StdinLock<'a> {
    fn as_raw_handle(&self) -> RawHandle {
        stdio_handle(unsafe { sys::c::GetStdHandle(sys::c::STD_INPUT_HANDLE) as RawHandle })
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=10 | LINES=7

```rust
#[stable(feature = "asraw_stdio_locks", since = "1.35.0")]
impl<'a> AsRawHandle for io::StdoutLock<'a> {
    fn as_raw_handle(&self) -> RawHandle {
        stdio_handle(unsafe { sys::c::GetStdHandle(sys::c::STD_OUTPUT_HANDLE) as RawHandle })
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=10 | LINES=7

```rust
#[stable(feature = "asraw_stdio_locks", since = "1.35.0")]
impl<'a> AsRawHandle for io::StderrLock<'a> {
    fn as_raw_handle(&self) -> RawHandle {
        stdio_handle(unsafe { sys::c::GetStdHandle(sys::c::STD_ERROR_HANDLE) as RawHandle })
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=stdio_handle | COMPLEXITY=7 | LINES=12

```rust
// Translate a handle returned from `GetStdHandle` into a handle to return to
// the user.
fn stdio_handle(raw: RawHandle) -> RawHandle {
    // `GetStdHandle` isn't expected to actually fail, so when it returns
    // `INVALID_HANDLE_VALUE`, it means we were launched from a parent which
    // didn't provide us with stdio handles, such as a parent with a detached
    // console. In that case, return null to the user, which is consistent
    // with what they'd get in the parent, and which avoids the problem that
    // `INVALID_HANDLE_VALUE` aliases the current process handle.
    if raw == sys::c::INVALID_HANDLE_VALUE { ptr::null_mut() } else { raw }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=13

```rust
#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawHandle for fs::File {
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> fs::File {
        unsafe {
            let handle = handle as sys::c::HANDLE;
            fs::File::from_inner(sys::fs::File::from_inner(FromInner::from_inner(
                OwnedHandle::from_raw_handle(handle),
            )))
        }
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=into_raw_handle | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawHandle for fs::File {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        self.into_inner().into_raw_handle() as *mut _
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=as_raw_socket | COMPLEXITY=7 | LINES=16

```rust
/// Extracts raw sockets.
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRawSocket {
    /// Extracts the raw socket.
    ///
    /// This function is typically used to **borrow** an owned socket.
    /// When used in this way, this method does **not** pass ownership of the
    /// raw socket to the caller, and the socket is only guaranteed
    /// to be valid while the original object has not yet been destroyed.
    ///
    /// However, borrowing is not strictly required. See [`AsSocket::as_socket`]
    /// for an API which strictly borrows a socket.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_raw_socket(&self) -> RawSocket;
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=27

```rust
/// Creates I/O objects from raw sockets.
#[stable(feature = "from_raw_os", since = "1.1.0")]
pub trait FromRawSocket {
    /// Constructs a new I/O object from the specified raw socket.
    ///
    /// This function is typically used to **consume ownership** of the socket
    /// given, passing responsibility for closing the socket to the returned
    /// object. When used in this way, the returned object
    /// will take responsibility for closing it when the object goes out of
    /// scope.
    ///
    /// However, consuming ownership is not strictly required. Use a
    /// `From<OwnedSocket>::from` implementation for an API which strictly
    /// consumes ownership.
    ///
    /// # Safety
    ///
    /// The `socket` passed in must:
    ///   - be an [owned socket][io-safety]; in particular, it must be open.
    ///   - be a socket that may be freed via [`closesocket`].
    ///
    /// [`closesocket`]: https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-closesocket
    /// [io-safety]: io#io-safety
    #[stable(feature = "from_raw_os", since = "1.1.0")]
    unsafe fn from_raw_socket(sock: RawSocket) -> Self;
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=into_raw_socket | COMPLEXITY=5 | LINES=18

```rust
/// A trait to express the ability to consume an object and acquire ownership of
/// its raw `SOCKET`.
#[stable(feature = "into_raw_os", since = "1.4.0")]
pub trait IntoRawSocket {
    /// Consumes this object, returning the raw underlying socket.
    ///
    /// This function is typically used to **transfer ownership** of the underlying
    /// socket to the caller. When used in this way, callers are then the unique
    /// owners of the socket and must close it once it's no longer needed.
    ///
    /// However, transferring ownership is not strictly required. Use a
    /// `Into<OwnedSocket>::into` implementation for an API which strictly
    /// transfers ownership.
    #[must_use = "losing the raw socket may leak resources"]
    #[stable(feature = "into_raw_os", since = "1.4.0")]
    fn into_raw_socket(self) -> RawSocket;
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=as_raw_socket | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawSocket for net::TcpStream {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.as_inner().socket().as_raw_socket()
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=as_raw_socket | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawSocket for net::TcpListener {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.as_inner().socket().as_raw_socket()
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=as_raw_socket | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawSocket for net::UdpSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.as_inner().socket().as_raw_socket()
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=11

```rust
#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawSocket for net::TcpStream {
    #[inline]
    unsafe fn from_raw_socket(sock: RawSocket) -> net::TcpStream {
        unsafe {
            let sock = sys::net::Socket::from_inner(OwnedSocket::from_raw_socket(sock));
            net::TcpStream::from_inner(sys::net::TcpStream::from_inner(sock))
        }
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=10

```rust
#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawSocket for net::TcpListener {
    #[inline]
    unsafe fn from_raw_socket(sock: RawSocket) -> net::TcpListener {
        unsafe {
            let sock = sys::net::Socket::from_inner(OwnedSocket::from_raw_socket(sock));
            net::TcpListener::from_inner(sys::net::TcpListener::from_inner(sock))
        }
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=10

```rust
#[stable(feature = "from_raw_os", since = "1.1.0")]
impl FromRawSocket for net::UdpSocket {
    #[inline]
    unsafe fn from_raw_socket(sock: RawSocket) -> net::UdpSocket {
        unsafe {
            let sock = sys::net::Socket::from_inner(OwnedSocket::from_raw_socket(sock));
            net::UdpSocket::from_inner(sys::net::UdpSocket::from_inner(sock))
        }
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=into_raw_socket | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawSocket for net::TcpStream {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.into_inner().into_socket().into_inner().into_raw_socket()
    }
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=FUNCTION | NAME=into_raw_socket | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawSocket for net::TcpListener {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.into_inner().into_socket().into_inner().into_raw_socket()
    }
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=FUNCTION | NAME=into_raw_socket | COMPLEXITY=5 | LINES=8

```rust
#[stable(feature = "into_raw_os", since = "1.4.0")]
impl IntoRawSocket for net::UdpSocket {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        self.into_inner().into_socket().into_inner().into_raw_socket()
    }
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
impl AsRawHandle for io::PipeReader {
    fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=7

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
impl FromRawHandle for io::PipeReader {
    unsafe fn from_raw_handle(raw_handle: RawHandle) -> Self {
        unsafe { Self::from_inner(FromRawHandle::from_raw_handle(raw_handle)) }
    }
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=FUNCTION | NAME=into_raw_handle | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
impl IntoRawHandle for io::PipeReader {
    fn into_raw_handle(self) -> RawHandle {
        self.0.into_raw_handle()
    }
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=FUNCTION | NAME=as_raw_handle | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
impl AsRawHandle for io::PipeWriter {
    fn as_raw_handle(&self) -> RawHandle {
        self.0.as_raw_handle()
    }
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=14 | LINES=7

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
impl FromRawHandle for io::PipeWriter {
    unsafe fn from_raw_handle(raw_handle: RawHandle) -> Self {
        unsafe { Self::from_inner(FromRawHandle::from_raw_handle(raw_handle)) }
    }
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=FUNCTION | NAME=into_raw_handle | COMPLEXITY=5 | LINES=7

```rust
#[stable(feature = "anonymous_pipe", since = "1.87.0")]
impl IntoRawHandle for io::PipeWriter {
    fn into_raw_handle(self) -> RawHandle {
        self.0.into_raw_handle()
    }
}
```

---
*Generated by AST tracing system*
