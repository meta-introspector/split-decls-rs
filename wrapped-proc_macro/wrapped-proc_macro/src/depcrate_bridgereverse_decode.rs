// Generated macro for reverse_decode (macro)
macro_rules! Depcrate_bridgereverse_decode {
() => {
// Module: crate::bridge
// Provides: {"reverse_decode"}
// Dependencies: {}
macro_rules ! reverse_decode { ($ reader : ident , $ s : ident ;) => { } ; ($ reader : ident , $ s : ident ; $ first : ident : $ first_ty : ty $ (, $ rest : ident : $ rest_ty : ty) *) => { reverse_decode ! ($ reader , $ s ; $ ($ rest : $ rest_ty) ,*) ; let $ first = <$ first_ty >:: decode (& mut $ reader , $ s) ; } }
};
}
