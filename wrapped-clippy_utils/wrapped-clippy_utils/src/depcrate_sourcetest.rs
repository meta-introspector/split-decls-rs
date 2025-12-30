// Generated macro for test (module)
macro_rules! Depcrate_sourcetest {
() => {
// Module: crate::source
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: reindent_multiline ; # [test] fn test_reindent_multiline_single_line () { assert_eq ! ("" , reindent_multiline ("" , false , None)) ; assert_eq ! ("..." , reindent_multiline ("..." , false , None)) ; assert_eq ! ("..." , reindent_multiline ("    ..." , false , None)) ; assert_eq ! ("..." , reindent_multiline ("\t..." , false , None)) ; assert_eq ! ("..." , reindent_multiline ("\t\t..." , false , None)) ; } # [test] # [rustfmt :: skip] fn test_reindent_multiline_block () { assert_eq ! ("\
    if x {
        y
    } else {
        z
    }" , reindent_multiline ("    if x {
            y
        } else {
            z
        }" , false , None)) ; assert_eq ! ("\
    if x {
    \ty
    } else {
    \tz
    }" , reindent_multiline ("    if x {
        \ty
        } else {
        \tz
        }" , false , None)) ; } # [test] # [rustfmt :: skip] fn test_reindent_multiline_empty_line () { assert_eq ! ("\
    if x {
        y

    } else {
        z
    }" , reindent_multiline ("    if x {
            y

        } else {
            z
        }" , false , None)) ; } # [test] # [rustfmt :: skip] fn test_reindent_multiline_lines_deeper () { assert_eq ! ("\
        if x {
            y
        } else {
            z
        }" , reindent_multiline ("\
    if x {
        y
    } else {
        z
    }" , true , Some (8))) ; } }
};
}
