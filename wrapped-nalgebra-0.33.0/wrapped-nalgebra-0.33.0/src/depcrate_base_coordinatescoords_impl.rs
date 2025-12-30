// Generated macro for coords_impl (macro)
macro_rules! Depcrate_base_coordinatescoords_impl {
() => {
// Module: crate::base::coordinates
// Provides: {"coords_impl"}
// Dependencies: {}
macro_rules ! coords_impl (($ T : ident ; $ ($ comps : ident) ,*) => { # [doc = " Data structure used to provide access to matrix and vector coordinates with the dot"] # [doc = " notation, e.g., `v.x` is the same as `v[0]` for a vector."] # [repr (C)] # [derive (Eq , PartialEq , Clone , Hash , Debug , Copy)] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] pub struct $ T < T : Scalar > { $ (pub $ comps : T) ,* } }) ;
};
}
