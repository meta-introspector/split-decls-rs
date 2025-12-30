// Generated macro for data_struct (macro)
macro_rules! Depcrate_varule_traitsdata_struct {
() => {
// Module: crate::varule_traits
// Provides: {"data_struct"}
// Dependencies: {}
# [doc = " Implements required traits on data structs, such as [`MaybeEncodeAsVarULE`]."] # [macro_export] macro_rules ! data_struct { (<$ generic : ident : $ bound : tt > $ ty : path $ (, $ (# [$ attr : meta]) *) ?) => { impl <$ generic : $ bound > $ crate :: ule :: MaybeAsVarULE for $ ty { type EncodedStruct = [()] ; } $ ($ (# [$ attr]) *) ? impl <$ generic : $ bound > $ crate :: ule :: MaybeEncodeAsVarULE for $ ty { fn maybe_encode_as_varule (& self) -> Option <& Self :: EncodedStruct > { None } } } ; ($ ty : path $ (, $ (# [$ attr : meta]) *) ?) => { impl $ crate :: ule :: MaybeAsVarULE for $ ty { type EncodedStruct = [()] ; } $ ($ (# [$ attr]) *) ? impl $ crate :: ule :: MaybeEncodeAsVarULE for $ ty { fn maybe_encode_as_varule (& self) -> Option <& Self :: EncodedStruct > { None } } } ; ($ ty : ty , varule : $ varule : ty , $ (# [$ attr : meta]) * encode_as_varule : $ encode_as_varule : expr) => { impl <'data > $ crate :: ule :: MaybeAsVarULE for $ ty { type EncodedStruct = $ varule ; } $ (# [$ attr]) * impl <'data > $ crate :: ule :: MaybeEncodeAsVarULE for $ ty { fn maybe_encode_as_varule (& self) -> Option <& Self :: EncodedStruct > { fn bind_lifetimes < F > (f : F) -> F where F : for <'data > Fn (&'data $ ty) -> &'data $ varule { f } Some (bind_lifetimes ($ encode_as_varule) (self)) } } } ; }
};
}
