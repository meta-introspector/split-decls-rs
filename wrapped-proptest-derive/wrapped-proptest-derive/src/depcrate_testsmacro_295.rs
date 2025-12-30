// Generated macro for macro_295 (macro)
macro_rules! Depcrate_testsmacro_295 {
() => {
// Module: crate::tests
// Provides: {"macro_295"}
// Dependencies: {}
test ! { struct_unit_named { # [derive (Debug)] struct MyNamedUnitStruct { } } expands to { # [allow (non_local_definitions)] # [allow (non_upper_case_globals)] # [allow (clippy :: arc_with_non_send_sync)] const _ : () = { use proptest as _proptest ; impl _proptest :: arbitrary :: Arbitrary for MyNamedUnitStruct { type Parameters = () ; type Strategy = fn () -> Self ; fn arbitrary_with (_top : Self :: Parameters) -> Self :: Strategy { (|| MyNamedUnitStruct { }) as fn () -> _ } } } ; } }
};
}
