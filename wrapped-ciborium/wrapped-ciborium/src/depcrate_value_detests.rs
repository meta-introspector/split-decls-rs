// Generated macro for tests (module)
macro_rules! Depcrate_value_detests {
() => {
// Module: crate::value::de
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn deserialize_bytes_from_array () { let value = Value :: serialized (& [1 , 2 , 3 , 4]) . unwrap () ; assert ! (value . is_array ()) ; let bytes = value . deserialized :: < serde_bytes :: ByteArray < 4 > > () . unwrap () ; assert_eq ! (bytes , & [1 , 2 , 3 , 4]) ; } }
};
}
