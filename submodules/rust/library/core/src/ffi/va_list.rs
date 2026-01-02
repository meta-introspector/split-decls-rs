mkuse!{use crate :: ffi :: c_void ;}
mkuse!{# [allow (unused_imports)] use crate :: fmt ;}
mkuse!{use crate :: intrinsics :: { va_arg , va_copy , va_end } ;}
mkuse!{use crate :: marker :: { PhantomData , PhantomInvariantLifetime } ;}
mkuse!{use crate :: ops :: { Deref , DerefMut } ;}
mkitem!{crate :: cfg_select ! { all (target_arch = "aarch64" , not (target_vendor = "apple") , not (target_os = "uefi") , not (windows) ,) => { # [doc = " AArch64 ABI implementation of a `va_list`. See the"] # [doc = " [AArch64 Procedure Call Standard] for more details."] # [doc = ""] # [doc = " [AArch64 Procedure Call Standard]:"] # [doc = " http://infocenter.arm.com/help/topic/com.arm.doc.ihi0055b/IHI0055B_aapcs64.pdf"] # [cfg_attr (not (doc) , repr (C))] # [derive (Debug)] # [lang = "va_list"] pub struct VaListImpl <'f > { stack : * mut c_void , gr_top : * mut c_void , vr_top : * mut c_void , gr_offs : i32 , vr_offs : i32 , _marker : PhantomInvariantLifetime <'f >, } } all (target_arch = "powerpc" , not (target_os = "uefi") , not (windows)) => { # [doc = " PowerPC ABI implementation of a `va_list`."] # [cfg_attr (not (doc) , repr (C))] # [derive (Debug)] # [lang = "va_list"] pub struct VaListImpl <'f > { gpr : u8 , fpr : u8 , reserved : u16 , overflow_arg_area : * mut c_void , reg_save_area : * mut c_void , _marker : PhantomInvariantLifetime <'f >, } } target_arch = "s390x" => { # [doc = " s390x ABI implementation of a `va_list`."] # [cfg_attr (not (doc) , repr (C))] # [derive (Debug)] # [lang = "va_list"] pub struct VaListImpl <'f > { gpr : i64 , fpr : i64 , overflow_arg_area : * mut c_void , reg_save_area : * mut c_void , _marker : PhantomInvariantLifetime <'f >, } } all (target_arch = "x86_64" , not (target_os = "uefi") , not (windows)) => { # [doc = " x86_64 ABI implementation of a `va_list`."] # [cfg_attr (not (doc) , repr (C))] # [derive (Debug)] # [lang = "va_list"] pub struct VaListImpl <'f > { gp_offset : i32 , fp_offset : i32 , overflow_arg_area : * mut c_void , reg_save_area : * mut c_void , _marker : PhantomInvariantLifetime <'f >, } } target_arch = "xtensa" => { # [doc = " Xtensa ABI implementation of a `va_list`."] # [repr (C)] # [derive (Debug)] # [lang = "va_list"] pub struct VaListImpl <'f > { stk : * mut i32 , reg : * mut i32 , ndx : i32 , _marker : PhantomInvariantLifetime <'f >, } } _ => { # [doc = " Basic implementation of a `va_list`."] # [repr (transparent)] # [lang = "va_list"] pub struct VaListImpl <'f > { ptr : * mut c_void , _marker : PhantomInvariantLifetime <'f >, } impl <'f > fmt :: Debug for VaListImpl <'f > { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { write ! (f , "va_list* {:p}" , self . ptr) } } } }}
mkitem!{crate :: cfg_select ! { all (any (target_arch = "aarch64" , target_arch = "powerpc" , target_arch = "s390x" , target_arch = "x86_64") , not (target_arch = "xtensa") , any (not (target_arch = "aarch64") , not (target_vendor = "apple")) , not (target_family = "wasm") , not (target_os = "uefi") , not (windows) ,) => { # [doc = " A wrapper for a `va_list`"] # [repr (transparent)] # [derive (Debug)] pub struct VaList <'a , 'f : 'a > { inner : &'a mut VaListImpl <'f >, _marker : PhantomData <&'a mut VaListImpl <'f >>, } impl <'f > VaListImpl <'f > { # [doc = " Converts a [`VaListImpl`] into a [`VaList`] that is binary-compatible with C's `va_list`."] # [inline] pub fn as_va_list <'a > (&'a mut self) -> VaList <'a , 'f > { VaList { inner : self , _marker : PhantomData } } } } _ => { # [doc = " A wrapper for a `va_list`"] # [repr (transparent)] # [derive (Debug)] pub struct VaList <'a , 'f : 'a > { inner : VaListImpl <'f >, _marker : PhantomData <&'a mut VaListImpl <'f >>, } impl <'f > VaListImpl <'f > { # [doc = " Converts a [`VaListImpl`] into a [`VaList`] that is binary-compatible with C's `va_list`."] # [inline] pub fn as_va_list <'a > (&'a mut self) -> VaList <'a , 'f > { VaList { inner : VaListImpl { ..* self } , _marker : PhantomData } } } } }}
mkitem!{mkimpl!{impl < 'a , 'f : 'a > Deref for VaList < 'a , 'f > { type Target = VaListImpl < 'f > ; # [inline] fn deref (& self) -> & VaListImpl < 'f > { & self . inner } }}}
mkitem!{mkimpl!{impl < 'a , 'f : 'a > DerefMut for VaList < 'a , 'f > { # [inline] fn deref_mut (& mut self) -> & mut VaListImpl < 'f > { & mut self . inner } }}}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkitem!{mktrait!{pub trait Sealed { }}}
mkitem!{mkimpl!{impl Sealed for i32 { }}}
mkitem!{mkimpl!{impl Sealed for i64 { }}}
mkitem!{mkimpl!{impl Sealed for isize { }}}
mkitem!{mkimpl!{impl Sealed for u32 { }}}
mkitem!{mkimpl!{impl Sealed for u64 { }}}
mkitem!{mkimpl!{impl Sealed for usize { }}}
mkitem!{mkimpl!{impl Sealed for f64 { }}}
mkitem!{mkimpl!{impl < T > Sealed for * mut T { }}}
mkitem!{mkimpl!{impl < T > Sealed for * const T { }}} 
            }}
mkitem!{mktrait!{# [doc = " Types that are valid to read using [`VaListImpl::arg`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The standard library implements this trait for primitive types that are"] # [doc = " expected to have a variable argument application-binary interface (ABI) on all"] # [doc = " platforms."] # [doc = ""] # [doc = " When C passes variable arguments, integers smaller than [`c_int`] and floats smaller"] # [doc = " than [`c_double`] are implicitly promoted to [`c_int`] and [`c_double`] respectively."] # [doc = " Implementing this trait for types that are subject to this promotion rule is invalid."] # [doc = ""] # [doc = " [`c_int`]: core::ffi::c_int"] # [doc = " [`c_double`]: core::ffi::c_double"] pub unsafe trait VaArgSafe : sealed :: Sealed { }}}
mkitem!{mkimpl!{unsafe impl VaArgSafe for i32 { }}}
mkitem!{mkimpl!{unsafe impl VaArgSafe for i64 { }}}
mkitem!{mkimpl!{unsafe impl VaArgSafe for isize { }}}
mkitem!{mkimpl!{unsafe impl VaArgSafe for u32 { }}}
mkitem!{mkimpl!{unsafe impl VaArgSafe for u64 { }}}
mkitem!{mkimpl!{unsafe impl VaArgSafe for usize { }}}
mkitem!{mkimpl!{unsafe impl VaArgSafe for f64 { }}}
mkitem!{mkimpl!{unsafe impl < T > VaArgSafe for * mut T { }}}
mkitem!{mkimpl!{unsafe impl < T > VaArgSafe for * const T { }}}
mkitem!{mkimpl!{impl < 'f > VaListImpl < 'f > { # [doc = " Advance to and read the next variable argument."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function is only sound to call when the next variable argument:"] # [doc = ""] # [doc = " - has a type that is ABI-compatible with the type `T`"] # [doc = " - has a value that is a properly initialized value of type `T`"] # [doc = ""] # [doc = " Calling this function with an incompatible type, an invalid value, or when there"] # [doc = " are no more variable arguments, is unsound."] # [doc = ""] # [doc = " [valid]: https://doc.rust-lang.org/nightly/nomicon/what-unsafe-does.html"] # [inline] pub unsafe fn arg < T : VaArgSafe > (& mut self) -> T { unsafe { va_arg (self) } } # [doc = " Copies the `va_list` at the current location."] pub unsafe fn with_copy < F , R > (& self , f : F) -> R where F : for < 'copy > FnOnce (VaList < 'copy , 'f >) -> R , { let mut ap = self . clone () ; let ret = f (ap . as_va_list ()) ; unsafe { va_end (& mut ap) ; } ret } }}}
mkitem!{mkimpl!{impl < 'f > Clone for VaListImpl < 'f > { # [inline] fn clone (& self) -> Self { let mut dest = crate :: mem :: MaybeUninit :: uninit () ; unsafe { va_copy (dest . as_mut_ptr () , self) ; dest . assume_init () } } }}}
mkitem!{mkimpl!{impl < 'f > Drop for VaListImpl < 'f > { fn drop (& mut self) { } }}}