// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__core_memtest {
() => {
// Module: crate::arbitrary::_core::mem
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { # [derive (Copy , Clone , Debug)] struct DummyStruct ; arbitrary ! (DummyStruct ; DummyStruct) ; no_panic_test ! (discriminant_struct => Discriminant < super :: DummyStruct >, discriminant_enum => Discriminant <:: std :: num :: FpCategory >) ; }
};
}
