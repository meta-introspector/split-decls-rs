// Generated macro for macro_293 (macro)
macro_rules! Depcrate_testsmacro_293 {
() => {
// Module: crate::tests
// Provides: {"macro_293"}
// Dependencies: {}
test ! { struct_unit_unit { # [derive (Debug)] struct MyUnitStruct ; } expands to { # [allow (non_local_definitions)] # [allow (non_upper_case_globals)] # [allow (clippy :: arc_with_non_send_sync)] const _ : () = { use proptest as _proptest ; impl _proptest :: arbitrary :: Arbitrary for MyUnitStruct { type Parameters = () ; type Strategy = fn () -> Self ; fn arbitrary_with (_top : Self :: Parameters) -> Self :: Strategy { (|| MyUnitStruct { }) as fn () -> _ } } } ; } }
};
}
