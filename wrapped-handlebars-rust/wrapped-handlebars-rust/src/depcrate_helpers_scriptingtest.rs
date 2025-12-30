// Generated macro for test (module)
macro_rules! Depcrate_helpers_scriptingtest {
() => {
// Module: crate::helpers::scripting
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: json :: value :: { PathAndJson , ScopedJson } ; use rhai :: Engine ; # [test] fn test_dynamic_convert () { let j0 = json ! { [{ "name" : "tomcat" } , { "name" : "jetty" }] } ; let d0 = to_dynamic (j0) . unwrap () ; assert_eq ! ("array" , d0 . type_name ()) ; let j1 = json ! ({ "name" : "tomcat" , "value" : 4000 , }) ; let d1 = to_dynamic (j1) . unwrap () ; assert_eq ! ("map" , d1 . type_name ()) ; } # [test] fn test_to_json () { let d0 = Dynamic :: from ("tomcat" . to_owned ()) ; assert_eq ! (Json :: String ("tomcat" . to_owned ()) , from_dynamic ::< Json > (& d0) . unwrap ()) ; } # [test] fn test_script_helper_value_access () { let engine = Engine :: new () ; let script = "let plen = len(params); let p0 = params[0]; let hlen = len(hash); let hme = hash[\"me\"]; plen + \",\" + p0 + \",\" + hlen + \",\" + hme" ; let ast = engine . compile (script) . unwrap () ; let params = vec ! [PathAndJson :: new (None , ScopedJson :: Derived (json ! (true)))] ; let mut hash = BTreeMap :: new () ; hash . insert ("me" , PathAndJson :: new (None , ScopedJson :: Derived (json ! ("no"))) ,) ; hash . insert ("you" , PathAndJson :: new (None , ScopedJson :: Derived (json ! ("yes"))) ,) ; let result = call_script_helper (& params , & hash , & engine , & ast) . unwrap () . render () ; assert_eq ! ("1,true,2,no" , & result) ; } }
};
}
