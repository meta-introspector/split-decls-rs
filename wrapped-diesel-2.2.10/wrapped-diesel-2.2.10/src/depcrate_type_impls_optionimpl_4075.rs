// Generated macro for impl_4075 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4075 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4075"}
// Dependencies: {}
impl < T , DB > QueryableByName < DB > for Option < T > where DB : Backend , T : QueryableByName < DB > , { fn build < 'a > (row : & impl crate :: row :: NamedRow < 'a , DB >) -> deserialize :: Result < Self > { match T :: build (row) { Ok (v) => Ok (Some (v)) , Err (e) if e . is :: < crate :: result :: UnexpectedNullError > () => Ok (None) , Err (e) => Err (e) , } } }
};
}
