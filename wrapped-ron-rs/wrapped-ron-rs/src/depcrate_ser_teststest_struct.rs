// Generated macro for test_struct (function)
macro_rules! Depcrate_ser_teststest_struct {
() => {
// Module: crate::ser::tests
// Provides: {"test_struct"}
// Dependencies: {}
# [test] fn test_struct () { let my_struct = MyStruct { x : 4.0 , y : 7.0 } ; check_to_string_writer (& my_struct , "(x:4.0,y:7.0)" , "MyStruct(x: 4.0, y: 7.0)") ; check_to_string_writer (& NewType (42) , "(42)" , "NewType(42)") ; check_to_string_writer (& TupleStruct (2.0 , 5.0) , "(2.0,5.0)" , "TupleStruct(2.0, 5.0)") ; }
};
}
