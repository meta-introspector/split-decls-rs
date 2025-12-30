// Generated macro for impl_1269 (impl)
macro_rules! Depcrate_runtime_selimpl_1269 {
() => {
// Module: crate::runtime::sel
// Provides: {"impl_1269"}
// Dependencies: {}
impl Sel { # [inline] # [doc (hidden)] pub const unsafe fn __internal_from_ptr (ptr : * const u8) -> Self { let ptr = unsafe { NonNull :: new_unchecked (ptr as * mut c_void) } ; Self { ptr } } # [inline] pub (crate) unsafe fn from_ptr (ptr : * const c_void) -> Option < Self > { NonNull :: new (ptr as * mut c_void) . map (| ptr | Self { ptr }) } # [inline] pub (crate) const fn as_ptr (& self) -> * const c_void { self . ptr . as_ptr () } pub (crate) unsafe fn register_unchecked (name : * const c_char) -> Self { let ptr = unsafe { ffi :: sel_registerName (name) } ; ptr . expect ("failed allocating selector") } # [doc = " Registers a selector with the Objective-C runtime."] # [doc = ""] # [doc = " This is the dynamic version of the [`sel!`] macro, prefer to use that"] # [doc = " when your selector is static."] # [doc = ""] # [doc = " [`sel!`]: crate::sel"] # [doc = ""] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the runtime failed allocating space for the selector."] # [inline] # [doc (alias = "sel_registerName")] pub fn register (name : & CStr) -> Self { unsafe { Self :: register_unchecked (name . as_ptr ()) } } # [doc = " Returns the string representation of the selector."] # [inline] # [doc (alias = "sel_getName")] pub fn name (self) -> & 'static CStr { let ptr = unsafe { ffi :: sel_getName (self) } ; unsafe { CStr :: from_ptr (ptr) } } pub (crate) fn number_of_arguments (self) -> usize { self . name () . to_bytes () . iter () . filter (| & & b | b == b':') . count () } }
};
}
