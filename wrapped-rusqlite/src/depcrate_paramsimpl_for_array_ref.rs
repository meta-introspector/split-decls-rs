// Generated macro for impl_for_array_ref (macro)
macro_rules! Depcrate_paramsimpl_for_array_ref {
() => {
// Module: crate::params
// Provides: {"impl_for_array_ref"}
// Dependencies: {}
macro_rules ! impl_for_array_ref { ($ ($ N : literal) +) => { $ (impl < T : ToSql + ? Sized > Sealed for & [& T ; $ N] { } impl < T : ToSql + ? Sized > Params for & [& T ; $ N] { fn __bind_in (self , stmt : & mut Statement <'_ >) -> Result < () > { stmt . bind_parameters (self) } } impl < S : BindIndex , T : ToSql + ? Sized > Sealed for & [(S , & T) ; $ N] { } impl < S : BindIndex , T : ToSql + ? Sized > Params for & [(S , & T) ; $ N] { fn __bind_in (self , stmt : & mut Statement <'_ >) -> Result < () > { stmt . bind_parameters_named (self) } } impl < T : ToSql > Sealed for [T ; $ N] { } impl < T : ToSql > Params for [T ; $ N] { # [inline] fn __bind_in (self , stmt : & mut Statement <'_ >) -> Result < () > { stmt . bind_parameters (& self) } }) + } ; }
};
}
