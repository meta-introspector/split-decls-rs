// Generated macro for serialize_to_from_value (function)
macro_rules! Depcrate_serde_testsserialize_to_from_value {
() => {
// Module: crate::serde_tests
// Provides: {"serialize_to_from_value"}
// Dependencies: {}
# [test] fn serialize_to_from_value () { let dog = Animal :: Dog (DogOuter { inner : vec ! [DogInner { a : () , b : 12 , c : vec ! ["a" . to_string () , "b" . to_string ()] , d : Some (Uid :: new (42)) , e : Data :: new (vec ! [1 , 2 , 3]) , }] , }) ; let dog_value = to_value (& dog) . unwrap () ; assert_eq ! (dog_value . as_dictionary () . unwrap () . get ("Dog") . unwrap () . as_dictionary () . unwrap () . get ("inner") . unwrap () . as_array () . unwrap () [0] . as_dictionary () . unwrap () ["b"] . as_unsigned_integer () . unwrap () , 12) ; let dog_roundtrip : Animal = from_value (& dog_value) . unwrap () ; assert_eq ! (dog_roundtrip , dog) ; }
};
}
