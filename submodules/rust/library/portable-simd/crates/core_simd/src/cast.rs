mkuse!{use crate :: simd :: SimdElement ;}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkitem!{mktrait!{# [doc = " Cast vector elements to other types."] # [doc = ""] # [doc = " # Safety"] # [doc = " Implementing this trait asserts that the type is a valid vector element for the `simd_cast`"] # [doc = " or `simd_as` intrinsics."] pub unsafe trait Sealed { }}} 
            }}
mkuse!{use sealed :: Sealed ;}
mkitem!{mktrait!{# [doc = " Supporting trait for `Simd::cast`.  Typically doesn't need to be used directly."] pub trait SimdCast : Sealed + SimdElement { }}}
mkitem!{mkimpl!{unsafe impl Sealed for i8 { }}}
mkitem!{mkimpl!{impl SimdCast for i8 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for i16 { }}}
mkitem!{mkimpl!{impl SimdCast for i16 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for i32 { }}}
mkitem!{mkimpl!{impl SimdCast for i32 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for i64 { }}}
mkitem!{mkimpl!{impl SimdCast for i64 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for isize { }}}
mkitem!{mkimpl!{impl SimdCast for isize { }}}
mkitem!{mkimpl!{unsafe impl Sealed for u8 { }}}
mkitem!{mkimpl!{impl SimdCast for u8 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for u16 { }}}
mkitem!{mkimpl!{impl SimdCast for u16 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for u32 { }}}
mkitem!{mkimpl!{impl SimdCast for u32 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for u64 { }}}
mkitem!{mkimpl!{impl SimdCast for u64 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for usize { }}}
mkitem!{mkimpl!{impl SimdCast for usize { }}}
mkitem!{mkimpl!{unsafe impl Sealed for f32 { }}}
mkitem!{mkimpl!{impl SimdCast for f32 { }}}
mkitem!{mkimpl!{unsafe impl Sealed for f64 { }}}
mkitem!{mkimpl!{impl SimdCast for f64 { }}}