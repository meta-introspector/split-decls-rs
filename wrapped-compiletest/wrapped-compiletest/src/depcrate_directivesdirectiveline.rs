// Generated macro for DirectiveLine (struct)
macro_rules! Depcrate_directivesDirectiveLine {
() => {
// Module: crate::directives
// Provides: {"DirectiveLine"}
// Dependencies: {}
# [doc = " The (partly) broken-down contents of a line containing a test directive,"] # [doc = " which [`iter_directives`] passes to its callback function."] # [doc = ""] # [doc = " For example:"] # [doc = ""] # [doc = " ```text"] # [doc = " //@ compile-flags: -O"] # [doc = "     ^^^^^^^^^^^^^^^^^ raw_directive"] # [doc = ""] # [doc = " //@ [foo] compile-flags: -O"] # [doc = "      ^^^                    revision"] # [doc = "           ^^^^^^^^^^^^^^^^^ raw_directive"] # [doc = " ```"] struct DirectiveLine < 'ln > { line_number : usize , # [doc = " Some test directives start with a revision name in square brackets"] # [doc = " (e.g. `[foo]`), and only apply to that revision of the test."] # [doc = " If present, this field contains the revision name (e.g. `foo`)."] revision : Option < & 'ln str > , # [doc = " The main part of the directive, after removing the comment prefix"] # [doc = " and the optional revision specifier."] # [doc = ""] # [doc = " This is \"raw\" because the directive's name and colon-separated value"] # [doc = " (if present) have not yet been extracted or checked."] raw_directive : & 'ln str , }
};
}
