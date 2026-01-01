# AST Trace: ../rust/library/core/src/ffi/va_list.rs

Generated 21 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
//! C's "variable arguments"
//!
//! Better known as "varargs".

use crate::ffi::c_void;
#[allow(unused_imports)]
use crate::fmt;
use crate::intrinsics::{va_arg, va_copy, va_end};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::marker::{PhantomData, PhantomInvariantLifetime};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::ops::{Deref, DerefMut};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=VaListImpl | COMPLEXITY=34 | LINES=109

```rust
// The name is WIP, using `VaListImpl` for now.
//
// Most targets explicitly specify the layout of `va_list`, this layout is matched here.
crate::cfg_select! {
    all(
        target_arch = "aarch64",
        not(target_vendor = "apple"),
        not(target_os = "uefi"),
        not(windows),
    ) => {
        /// AArch64 ABI implementation of a `va_list`. See the
        /// [AArch64 Procedure Call Standard] for more details.
        ///
        /// [AArch64 Procedure Call Standard]:
        /// http://infocenter.arm.com/help/topic/com.arm.doc.ihi0055b/IHI0055B_aapcs64.pdf
        #[cfg_attr(not(doc), repr(C))] // work around https://github.com/rust-lang/rust/issues/66401
        #[derive(Debug)]
        #[lang = "va_list"]
        pub struct VaListImpl<'f> {
            stack: *mut c_void,
            gr_top: *mut c_void,
            vr_top: *mut c_void,
            gr_offs: i32,
            vr_offs: i32,
            _marker: PhantomInvariantLifetime<'f>,
        }
    }
    all(target_arch = "powerpc", not(target_os = "uefi"), not(windows)) => {
        /// PowerPC ABI implementation of a `va_list`.
        #[cfg_attr(not(doc), repr(C))] // work around https://github.com/rust-lang/rust/issues/66401
        #[derive(Debug)]
        #[lang = "va_list"]
        pub struct VaListImpl<'f> {
            gpr: u8,
            fpr: u8,
            reserved: u16,
            overflow_arg_area: *mut c_void,
            reg_save_area: *mut c_void,
            _marker: PhantomInvariantLifetime<'f>,
        }
    }
    target_arch = "s390x" => {
        /// s390x ABI implementation of a `va_list`.
        #[cfg_attr(not(doc), repr(C))] // work around https://github.com/rust-lang/rust/issues/66401
        #[derive(Debug)]
        #[lang = "va_list"]
        pub struct VaListImpl<'f> {
            gpr: i64,
            fpr: i64,
            overflow_arg_area: *mut c_void,
            reg_save_area: *mut c_void,
            _marker: PhantomInvariantLifetime<'f>,
        }
    }
    all(target_arch = "x86_64", not(target_os = "uefi"), not(windows)) => {
        /// x86_64 ABI implementation of a `va_list`.
        #[cfg_attr(not(doc), repr(C))] // work around https://github.com/rust-lang/rust/issues/66401
        #[derive(Debug)]
        #[lang = "va_list"]
        pub struct VaListImpl<'f> {
            gp_offset: i32,
            fp_offset: i32,
            overflow_arg_area: *mut c_void,
            reg_save_area: *mut c_void,
            _marker: PhantomInvariantLifetime<'f>,
        }
    }
    target_arch = "xtensa" => {
        /// Xtensa ABI implementation of a `va_list`.
        #[repr(C)]
        #[derive(Debug)]
        #[lang = "va_list"]
        pub struct VaListImpl<'f> {
            stk: *mut i32,
            reg: *mut i32,
            ndx: i32,
            _marker: PhantomInvariantLifetime<'f>,
        }
    }

    // The fallback implementation, used for:
    //
    // - apple aarch64 (see https://github.com/rust-lang/rust/pull/56599)
    // - windows
    // - uefi
    // - any other target for which we don't specify the `VaListImpl` above
    //
    // In this implementation the `va_list` type is just an alias for an opaque pointer.
    // That pointer is probably just the next variadic argument on the caller's stack.
    _ => {
        /// Basic implementation of a `va_list`.
        #[repr(transparent)]
        #[lang = "va_list"]
        pub struct VaListImpl<'f> {
            ptr: *mut c_void,

            // Invariant over `'f`, so each `VaListImpl<'f>` object is tied to
            // the region of the function it's defined in
            _marker: PhantomInvariantLifetime<'f>,
        }

        impl<'f> fmt::Debug for VaListImpl<'f> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "va_list* {:p}", self.ptr)
            }
        }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=VaList | COMPLEXITY=20 | LINES=51

```rust
crate::cfg_select! {
    all(
        any(
            target_arch = "aarch64",
            target_arch = "powerpc",
            target_arch = "s390x",
            target_arch = "x86_64"
        ),
        not(target_arch = "xtensa"),
        any(not(target_arch = "aarch64"), not(target_vendor = "apple")),
        not(target_family = "wasm"),
        not(target_os = "uefi"),
        not(windows),
    ) => {
        /// A wrapper for a `va_list`
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct VaList<'a, 'f: 'a> {
            inner: &'a mut VaListImpl<'f>,
            _marker: PhantomData<&'a mut VaListImpl<'f>>,
        }


        impl<'f> VaListImpl<'f> {
            /// Converts a [`VaListImpl`] into a [`VaList`] that is binary-compatible with C's `va_list`.
            #[inline]
            pub fn as_va_list<'a>(&'a mut self) -> VaList<'a, 'f> {
                VaList { inner: self, _marker: PhantomData }
            }
        }
    }

    _ => {
        /// A wrapper for a `va_list`
        #[repr(transparent)]
        #[derive(Debug)]
        pub struct VaList<'a, 'f: 'a> {
            inner: VaListImpl<'f>,
            _marker: PhantomData<&'a mut VaListImpl<'f>>,
        }

        impl<'f> VaListImpl<'f> {
            /// Converts a [`VaListImpl`] into a [`VaList`] that is binary-compatible with C's `va_list`.
            #[inline]
            pub fn as_va_list<'a>(&'a mut self) -> VaList<'a, 'f> {
                VaList { inner: VaListImpl { ..*self }, _marker: PhantomData }
            }
        }
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=5 | LINES=9

```rust
impl<'a, 'f: 'a> Deref for VaList<'a, 'f> {
    type Target = VaListImpl<'f>;

    #[inline]
    fn deref(&self) -> &VaListImpl<'f> {
        &self.inner
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=deref_mut | COMPLEXITY=5 | LINES=7

```rust
impl<'a, 'f: 'a> DerefMut for VaList<'a, 'f> {
    #[inline]
    fn deref_mut(&mut self) -> &mut VaListImpl<'f> {
        &mut self.inner
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=30 | LINES=17

```rust
mod sealed {
    pub trait Sealed {}

    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for isize {}

    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for usize {}

    impl Sealed for f64 {}

    impl<T> Sealed for *mut T {}
    impl<T> Sealed for *const T {}
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=11 | LINES=19

```rust
/// Types that are valid to read using [`VaListImpl::arg`].
///
/// # Safety
///
/// The standard library implements this trait for primitive types that are
/// expected to have a variable argument application-binary interface (ABI) on all
/// platforms.
///
/// When C passes variable arguments, integers smaller than [`c_int`] and floats smaller
/// than [`c_double`] are implicitly promoted to [`c_int`] and [`c_double`] respectively.
/// Implementing this trait for types that are subject to this promotion rule is invalid.
///
/// [`c_int`]: core::ffi::c_int
/// [`c_double`]: core::ffi::c_double
// We may unseal this trait in the future, but currently our `va_arg` implementations don't support
// types with an alignment larger than 8, or with a non-scalar layout. Inline assembly can be used
// to accept unsupported types in the meantime.
pub unsafe trait VaArgSafe: sealed::Sealed {}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
// i8 and i16 are implicitly promoted to c_int in C, and cannot implement `VaArgSafe`.
unsafe impl VaArgSafe for i32 {}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1

```rust
unsafe impl VaArgSafe for i64 {}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1

```rust
unsafe impl VaArgSafe for isize {}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
// u8 and u16 are implicitly promoted to c_int in C, and cannot implement `VaArgSafe`.
unsafe impl VaArgSafe for u32 {}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1

```rust
unsafe impl VaArgSafe for u64 {}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1

```rust
unsafe impl VaArgSafe for usize {}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=3

```rust
// f32 is implicitly promoted to c_double in C, and cannot implement `VaArgSafe`.
unsafe impl VaArgSafe for f64 {}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=2

```rust
unsafe impl<T> VaArgSafe for *mut T {}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=1

```rust
unsafe impl<T> VaArgSafe for *const T {}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=28 | LINES=35

```rust
impl<'f> VaListImpl<'f> {
    /// Advance to and read the next variable argument.
    ///
    /// # Safety
    ///
    /// This function is only sound to call when the next variable argument:
    ///
    /// - has a type that is ABI-compatible with the type `T`
    /// - has a value that is a properly initialized value of type `T`
    ///
    /// Calling this function with an incompatible type, an invalid value, or when there
    /// are no more variable arguments, is unsound.
    ///
    /// [valid]: https://doc.rust-lang.org/nightly/nomicon/what-unsafe-does.html
    #[inline]
    pub unsafe fn arg<T: VaArgSafe>(&mut self) -> T {
        // SAFETY: the caller must uphold the safety contract for `va_arg`.
        unsafe { va_arg(self) }
    }

    /// Copies the `va_list` at the current location.
    pub unsafe fn with_copy<F, R>(&self, f: F) -> R
    where
        F: for<'copy> FnOnce(VaList<'copy, 'f>) -> R,
    {
        let mut ap = self.clone();
        let ret = f(ap.as_va_list());
        // SAFETY: the caller must uphold the safety contract for `va_end`.
        unsafe {
            va_end(&mut ap);
        }
        ret
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=clone | COMPLEXITY=10 | LINES=12

```rust
impl<'f> Clone for VaListImpl<'f> {
    #[inline]
    fn clone(&self) -> Self {
        let mut dest = crate::mem::MaybeUninit::uninit();
        // SAFETY: we write to the `MaybeUninit`, thus it is initialized and `assume_init` is legal
        unsafe {
            va_copy(dest.as_mut_ptr(), self);
            dest.assume_init()
        }
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=8 | LINES=15

```rust
impl<'f> Drop for VaListImpl<'f> {
    fn drop(&mut self) {
        // FIXME: this should call `va_end`, but there's no clean way to
        // guarantee that `drop` always gets inlined into its caller,
        // so the `va_end` would get directly called from the same function as
        // the corresponding `va_copy`. `man va_end` states that C requires this,
        // and LLVM basically follows the C semantics, so we need to make sure
        // that `va_end` is always called from the same function as `va_copy`.
        // For more details, see https://github.com/rust-lang/rust/pull/59625
        // and https://llvm.org/docs/LangRef.html#llvm-va-end-intrinsic.
        //
        // This works for now, since `va_end` is a no-op on all current LLVM targets.
    }
}
```

---
*Generated by AST tracing system*
