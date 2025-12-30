// Generated macro for impl_170 (impl)
macro_rules! Depcrateimpl_170 {
() => {
// Module: crate
// Provides: {"impl_170"}
// Dependencies: {}
impl dyn EyreHandler { # [doc = " Check if the handler is of type `T`"] pub fn is < T : EyreHandler > (& self) -> bool { let t = core :: any :: TypeId :: of :: < T > () ; let concrete = self . type_id () ; t == concrete } # [doc = " Downcast the handler to a concrete type"] pub fn downcast_ref < T : EyreHandler > (& self) -> Option < & T > { if self . is :: < T > () { unsafe { Some (& * (self as * const dyn EyreHandler as * const T)) } } else { None } } # [doc = " Downcast the handler to a concrete type"] pub fn downcast_mut < T : EyreHandler > (& mut self) -> Option < & mut T > { if self . is :: < T > () { unsafe { Some (& mut * (self as * mut dyn EyreHandler as * mut T)) } } else { None } } }
};
}
