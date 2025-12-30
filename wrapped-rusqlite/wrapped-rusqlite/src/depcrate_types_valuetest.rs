// Generated macro for test (module)
macro_rules! Depcrate_types_valuetest {
() => {
// Module: crate::types::value
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: Value ; use crate :: types :: Type ; # [test] fn from () { assert_eq ! (Value :: from (2f32) , Value :: Real (2f64)) ; assert_eq ! (Value :: from (3.) , Value :: Real (3.)) ; assert_eq ! (Value :: from (vec ! [0u8]) , Value :: Blob (vec ! [0u8])) ; } # [test] fn data_type () { assert_eq ! (Value :: Null . data_type () , Type :: Null) ; assert_eq ! (Value :: Integer (0) . data_type () , Type :: Integer) ; assert_eq ! (Value :: Real (0.) . data_type () , Type :: Real) ; assert_eq ! (Value :: Text (String :: new ()) . data_type () , Type :: Text) ; assert_eq ! (Value :: Blob (vec ! []) . data_type () , Type :: Blob) ; } # [test] fn from_option () { assert_eq ! (Value :: from (None as Option < i64 >) , Value :: Null) ; assert_eq ! (Value :: from (Some (0)) , Value :: Integer (0)) ; } }
};
}
