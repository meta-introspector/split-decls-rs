// Generated macro for test (module)
macro_rules! Depcrate_data_valuetest {
() => {
// Module: crate::data_value
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn type_conversions () { assert_eq ! (DataValue :: V128 ([0 ; 16]) . ty () , types :: I8X16) ; assert_eq ! (TryInto ::< [u8 ; 16] >:: try_into (DataValue :: V128 ([0 ; 16])) . unwrap () , [0 ; 16]) ; assert_eq ! (TryInto ::< i32 >:: try_into (DataValue :: V128 ([0 ; 16])) . unwrap_err () , DataValueCastFailure :: TryInto (types :: I8X16 , types :: I32)) ; } }
};
}
