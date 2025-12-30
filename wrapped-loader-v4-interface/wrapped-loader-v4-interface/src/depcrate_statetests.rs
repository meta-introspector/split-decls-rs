// Generated macro for tests (module)
macro_rules! Depcrate_statetests {
() => {
// Module: crate::state
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: * , memoffset :: offset_of } ; # [test] fn test_layout () { assert_eq ! (offset_of ! (LoaderV4State , slot) , 0x00) ; assert_eq ! (offset_of ! (LoaderV4State , authority_address_or_next_version) , 0x08) ; assert_eq ! (offset_of ! (LoaderV4State , status) , 0x28) ; assert_eq ! (LoaderV4State :: program_data_offset () , 0x30) ; } }
};
}
