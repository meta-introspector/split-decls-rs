// Generated macro for tests (module)
macro_rules! Depcrate_restricted_namestests {
() => {
// Module: crate::restricted_names
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn valid_feature_names () { assert ! (validate_feature_name ("c++17") . is_ok ()) ; assert ! (validate_feature_name ("128bit") . is_ok ()) ; assert ! (validate_feature_name ("_foo") . is_ok ()) ; assert ! (validate_feature_name ("feat-name") . is_ok ()) ; assert ! (validate_feature_name ("feat_name") . is_ok ()) ; assert ! (validate_feature_name ("foo.bar") . is_ok ()) ; assert ! (validate_feature_name ("") . is_err ()) ; assert ! (validate_feature_name ("+foo") . is_err ()) ; assert ! (validate_feature_name ("-foo") . is_err ()) ; assert ! (validate_feature_name (".foo") . is_err ()) ; assert ! (validate_feature_name ("dep:bar") . is_err ()) ; assert ! (validate_feature_name ("foo/bar") . is_err ()) ; assert ! (validate_feature_name ("foo:bar") . is_err ()) ; assert ! (validate_feature_name ("foo?") . is_err ()) ; assert ! (validate_feature_name ("?foo") . is_err ()) ; assert ! (validate_feature_name ("ⒶⒷⒸ") . is_err ()) ; assert ! (validate_feature_name ("a¼") . is_err ()) ; assert ! (validate_feature_name ("") . is_err ()) ; } }
};
}
