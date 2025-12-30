// Generated macro for macro_294 (macro)
macro_rules! Depcrate_testsmacro_294 {
() => {
// Module: crate::tests
// Provides: {"macro_294"}
// Dependencies: {}
test ! { struct_unit_tuple { # [derive (Debug)] struct MyTupleUnitStruct () ; } expands to { # [allow (non_local_definitions)] # [allow (non_upper_case_globals)] # [allow (clippy :: arc_with_non_send_sync)] const _ : () = { use proptest as _proptest ; impl _proptest :: arbitrary :: Arbitrary for MyTupleUnitStruct { type Parameters = () ; type Strategy = fn () -> Self ; fn arbitrary_with (_top : Self :: Parameters) -> Self :: Strategy { (|| MyTupleUnitStruct { }) as fn () -> _ } } } ; } }
};
}
