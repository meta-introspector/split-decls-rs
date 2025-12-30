// Generated macro for impl_707 (impl)
macro_rules! Depcrate_ir_extfuncimpl_707 {
() => {
// Module: crate::ir::extfunc
// Provides: {"impl_707"}
// Dependencies: {}
impl AbiParam { # [doc = " Create a parameter with default flags."] pub fn new (vt : Type) -> Self { Self { value_type : vt , extension : ArgumentExtension :: None , purpose : ArgumentPurpose :: Normal , } } # [doc = " Create a special-purpose parameter that is not (yet) bound to a specific register."] pub fn special (vt : Type , purpose : ArgumentPurpose) -> Self { Self { value_type : vt , extension : ArgumentExtension :: None , purpose , } } # [doc = " Convert `self` to a parameter with the `uext` flag set."] pub fn uext (self) -> Self { debug_assert ! (self . value_type . is_int () , "uext on {} arg" , self . value_type) ; Self { extension : ArgumentExtension :: Uext , .. self } } # [doc = " Convert `self` to a parameter type with the `sext` flag set."] pub fn sext (self) -> Self { debug_assert ! (self . value_type . is_int () , "sext on {} arg" , self . value_type) ; Self { extension : ArgumentExtension :: Sext , .. self } } }
};
}
