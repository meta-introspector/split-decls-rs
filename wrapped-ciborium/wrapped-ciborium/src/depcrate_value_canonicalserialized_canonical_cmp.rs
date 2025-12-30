// Generated macro for serialized_canonical_cmp (function)
macro_rules! Depcrate_value_canonicalserialized_canonical_cmp {
() => {
// Module: crate::value::canonical
// Provides: {"serialized_canonical_cmp"}
// Dependencies: {}
# [doc = " Manually serialize values to compare them."] fn serialized_canonical_cmp (v1 : & Value , v2 : & Value) -> Ordering { let mut bytes1 = Vec :: new () ; let _ = crate :: ser :: into_writer (v1 , & mut bytes1) ; let mut bytes2 = Vec :: new () ; let _ = crate :: ser :: into_writer (v2 , & mut bytes2) ; match bytes1 . len () . cmp (& bytes2 . len ()) { Ordering :: Equal => bytes1 . cmp (& bytes2) , x => x , } }
};
}
