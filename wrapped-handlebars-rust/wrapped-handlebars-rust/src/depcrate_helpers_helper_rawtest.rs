// Generated macro for test (module)
macro_rules! Depcrate_helpers_helper_rawtest {
() => {
// Module: crate::helpers::helper_raw
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: registry :: Registry ; # [test] fn test_raw_helper () { let mut handlebars = Registry :: new () ; assert ! (handlebars . register_template_string ("t0" , "a{{{{raw}}}}{{content}}{{else}}hello{{{{/raw}}}}") . is_ok ()) ; let r = handlebars . render ("t0" , & ()) ; assert_eq ! (r . ok () . unwrap () , "a{{content}}{{else}}hello") ; } }
};
}
