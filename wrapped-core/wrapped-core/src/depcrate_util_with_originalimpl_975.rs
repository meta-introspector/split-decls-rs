// Generated macro for impl_975 (impl)
macro_rules! Depcrate_util_with_originalimpl_975 {
() => {
// Module: crate::util::with_original
// Provides: {"impl_975"}
// Dependencies: {}
# [doc = " Get the type param usage of `parsed`."] impl < P : UsesTypeParams , O > UsesTypeParams for WithOriginal < P , O > { fn uses_type_params < 'a > (& self , options : & crate :: usage :: Options , type_set : & 'a crate :: usage :: IdentSet ,) -> crate :: usage :: IdentRefSet < 'a > { self . parsed . uses_type_params (options , type_set) } }
};
}
