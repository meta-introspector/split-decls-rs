// Generated macro for rename (function)
macro_rules! Depcrate_de_testsrename {
() => {
// Module: crate::de::tests
// Provides: {"rename"}
// Dependencies: {}
# [test] fn rename () { # [derive (Deserialize , Debug , PartialEq)] enum Foo { # [serde (rename = "2d")] D2 , # [serde (rename = "triangle-list")] TriangleList , } check_from_str_bytes_reader ("r#2d" , Ok (Foo :: D2)) ; check_from_str_bytes_reader ("r#triangle-list" , Ok (Foo :: TriangleList)) ; }
};
}
