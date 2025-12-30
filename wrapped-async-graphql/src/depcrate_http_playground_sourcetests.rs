// Generated macro for tests (module)
macro_rules! Depcrate_http_playground_sourcetests {
() => {
// Module: crate::http::playground_source
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use indexmap :: IndexMap ; use super :: * ; # [test] fn test_with_setting_can_use_any_json_value () { let settings = GraphQLPlaygroundConfig :: new ("") . with_setting ("string" , "string") . with_setting ("bool" , false) . with_setting ("number" , 10) . with_setting ("null" , Value :: Null) . with_setting ("array" , Vec :: from ([1 , 2 , 3])) . with_setting ("object" , IndexMap :: new ()) ; let json = serde_json :: to_value (settings) . unwrap () ; let settings = json ["settings"] . as_object () . unwrap () ; assert ! (settings ["string"] . as_str () . is_some ()) ; assert ! (settings ["bool"] . as_bool () . is_some ()) ; assert ! (settings ["number"] . as_u64 () . is_some ()) ; assert ! (settings ["null"] . as_null () . is_some ()) ; assert ! (settings ["array"] . as_array () . is_some ()) ; assert ! (settings ["object"] . as_object () . is_some ()) ; } }
};
}
