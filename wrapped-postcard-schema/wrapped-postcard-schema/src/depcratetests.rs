// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn crate_path () { # [allow (unused)] # [derive (Schema)] # [postcard (crate = crate)] struct Point { x : i32 , y : i32 , } assert_eq ! (Point :: SCHEMA , & schema :: NamedType { name : "Point" , ty : & schema :: DataModelType :: Struct (& [& schema :: NamedValue { name : "x" , ty : i32 :: SCHEMA } , & schema :: NamedValue { name : "y" , ty : i32 :: SCHEMA } ,]) }) ; } }
};
}
