// Generated macro for test (module)
macro_rules! Depcrate_types_anytest {
() => {
// Module: crate::types::any
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_conversion_ok () { let value = Value :: List (vec ! [Value :: Number (1 . into ()) , Value :: Boolean (true) , Value :: Null ,]) ; let expected = Any (value . clone ()) ; let output : Any = value . into () ; assert_eq ! (output , expected) ; } }
};
}
