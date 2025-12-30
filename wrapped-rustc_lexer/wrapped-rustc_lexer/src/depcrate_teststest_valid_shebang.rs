// Generated macro for test_valid_shebang (function)
macro_rules! Depcrate_teststest_valid_shebang {
() => {
// Module: crate::tests
// Provides: {"test_valid_shebang"}
// Dependencies: {}
# [test] fn test_valid_shebang () { let input = "#!/bin/bash" ; assert_eq ! (strip_shebang (input) , Some (input . len ())) ; let input = "#![attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#!    /bin/bash" ; assert_eq ! (strip_shebang (input) , Some (input . len ())) ; let input = "#!    [attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#! /* blah */  /bin/bash" ; assert_eq ! (strip_shebang (input) , Some (input . len ())) ; let input = "#! /* blah */  [attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#! // blah\n/bin/bash" ; assert_eq ! (strip_shebang (input) , Some (10)) ; let input = "#! // blah\n[attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#! /* blah\nblah\nblah */  /bin/bash" ; assert_eq ! (strip_shebang (input) , Some (10)) ; let input = "#! /* blah\nblah\nblah */  [attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "#!\n/bin/sh" ; assert_eq ! (strip_shebang (input) , Some (2)) ; let input = "#!\n[attribute]" ; assert_eq ! (strip_shebang (input) , None) ; let input = "\n#!/bin/bash" ; assert_eq ! (strip_shebang (input) , None) ; let input = "\n#![attribute]" ; assert_eq ! (strip_shebang (input) , None) ; }
};
}
