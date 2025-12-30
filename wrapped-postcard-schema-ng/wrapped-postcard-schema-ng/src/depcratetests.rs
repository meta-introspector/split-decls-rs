// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn crate_path () { # [allow (unused)] # [derive (Schema)] # [postcard (crate = crate)] struct Point { x : i32 , y : i32 , } assert_eq ! (Point :: SCHEMA , & schema :: DataModelType :: Struct { name : "Point" , data : schema :: Data :: Struct (& [& schema :: NamedField { name : "x" , ty : i32 :: SCHEMA } , & schema :: NamedField { name : "y" , ty : i32 :: SCHEMA } ,]) }) ; } }
};
}
