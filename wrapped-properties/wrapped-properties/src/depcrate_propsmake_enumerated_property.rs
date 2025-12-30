// Generated macro for make_enumerated_property (macro)
macro_rules! Depcrate_propsmake_enumerated_property {
() => {
// Module: crate::props
// Provides: {"make_enumerated_property"}
// Dependencies: {}
macro_rules ! make_enumerated_property { (name : $ name : literal ; short_name : $ short_name : literal ; ident : $ value_ty : path ; data_marker : $ data_marker : ty ; singleton : $ singleton : ident ; $ (ule_ty : $ ule_ty : ty ;) ?) => { impl crate :: private :: Sealed for $ value_ty { } impl EnumeratedProperty for $ value_ty { type DataMarker = $ data_marker ; # [cfg (feature = "compiled_data")] const SINGLETON : &'static crate :: provider :: PropertyCodePointMap <'static , Self > = crate :: provider :: Baked ::$ singleton ; const NAME : &'static [u8] = $ name . as_bytes () ; const SHORT_NAME : &'static [u8] = $ short_name . as_bytes () ; } $ (impl zerovec :: ule :: AsULE for $ value_ty { type ULE = $ ule_ty ; fn to_unaligned (self) -> Self :: ULE { self . 0 . to_unaligned () } fn from_unaligned (unaligned : Self :: ULE) -> Self { Self (zerovec :: ule :: AsULE :: from_unaligned (unaligned)) } }) ? } ; }
};
}
