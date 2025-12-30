// Generated macro for without_defaults (function)
macro_rules! Depcrate_internals_genericswithout_defaults {
() => {
// Module: crate::internals::generics
// Provides: {"without_defaults"}
// Dependencies: {}
pub fn without_defaults (generics : & Generics) -> Generics { syn :: Generics { params : generics . params . iter () . map (| param | match param { syn :: GenericParam :: Type (param) => syn :: GenericParam :: Type (syn :: TypeParam { eq_token : None , default : None , .. param . clone () }) , _ => param . clone () , }) . collect () , .. generics . clone () } }
};
}
