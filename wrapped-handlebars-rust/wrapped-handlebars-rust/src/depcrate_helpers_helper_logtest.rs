// Generated macro for test (module)
macro_rules! Depcrate_helpers_helper_logtest {
() => {
// Module: crate::helpers::helper_log
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: registry :: Registry ; # [test] # [cfg (not (feature = "no_logging"))] fn test_log_helper () { let mut handlebars = Registry :: new () ; assert ! (handlebars . register_template_string ("t0" , "{{log this level=\"warn\"}}") . is_ok ()) ; assert ! (handlebars . register_template_string ("t1" , "{{log this level=\"hello\"}}") . is_ok ()) ; assert ! (handlebars . register_template_string ("t2" , "{{log this}}") . is_ok ()) ; let r0 = handlebars . render ("t0" , & true) ; assert ! (r0 . is_ok ()) ; let r1 = handlebars . render ("t1" , & true) ; assert ! (r1 . is_err ()) ; let r2 = handlebars . render ("t2" , & true) ; assert ! (r2 . is_ok ()) ; } # [test] # [cfg (feature = "no_logging")] fn test_log_helper () { let mut handlebars = Registry :: new () ; assert ! (handlebars . register_template_string ("t0" , "{{log this level=\"warn\"}}") . is_ok ()) ; assert ! (handlebars . register_template_string ("t1" , "{{log this level=\"hello\"}}") . is_ok ()) ; assert ! (handlebars . register_template_string ("t2" , "{{log this}}") . is_ok ()) ; let r0 = handlebars . render ("t0" , & true) ; assert ! (r0 . is_ok ()) ; let r1 = handlebars . render ("t1" , & true) ; assert ! (r1 . is_ok ()) ; let r2 = handlebars . render ("t2" , & true) ; assert ! (r2 . is_ok ()) ; } }
};
}
