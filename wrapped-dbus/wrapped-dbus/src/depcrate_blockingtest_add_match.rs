// Generated macro for test_add_match (function)
macro_rules! Depcrate_blockingtest_add_match {
() => {
// Module: crate::blocking
// Provides: {"test_add_match"}
// Dependencies: {}
# [test] fn test_add_match () { use self :: stdintf :: org_freedesktop_dbus :: PropertiesPropertiesChanged as Ppc ; let c = Connection :: new_session () . unwrap () ; let x = c . add_match (Ppc :: match_rule (None , None) , | _ : Ppc , _ , _ | { true }) . unwrap () ; c . remove_match (x) . unwrap () ; }
};
}
