// Generated macro for rename (function)
macro_rules! Depcrate_ser_testsrename {
() => {
// Module: crate::ser::tests
// Provides: {"rename"}
// Dependencies: {}
# [test] fn rename () { # [derive (Serialize , Debug , PartialEq)] enum Foo { # [serde (rename = "2d")] D2 , # [serde (rename = "triangle-list")] TriangleList , } check_to_string_writer (& Foo :: D2 , "r#2d" , "r#2d") ; check_to_string_writer (& Foo :: TriangleList , "r#triangle-list" , "r#triangle-list") ; }
};
}
