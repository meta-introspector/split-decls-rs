// Generated macro for callgrind_diff (function)
macro_rules! Depcrate_valgrindcallgrind_diff {
() => {
// Module: crate::valgrind
// Provides: {"callgrind_diff"}
// Dependencies: {}
# [doc = " Returns the detailed instruction diff between the baseline and the candidate"] pub (crate) fn callgrind_diff (baseline : & Path , candidate : & Path , scenario : & str ,) -> anyhow :: Result < String > { let callgrind_annotate_base = Command :: new ("callgrind_annotate") . arg (baseline . join (CALLGRIND_OUTPUT_SUBDIR) . join (scenario) ,) . arg ("--auto=no") . output () . context ("error waiting for callgrind_annotate to finish") ? ; let callgrind_annotate_candidate = Command :: new ("callgrind_annotate") . arg (candidate . join (CALLGRIND_OUTPUT_SUBDIR) . join (scenario) ,) . arg ("--auto=no") . output () . context ("error waiting for callgrind_annotate to finish") ? ; if ! callgrind_annotate_base . status . success () { anyhow :: bail ! ("callgrind_annotate for base finished with an error (code = {:?})" , callgrind_annotate_base . status . code ()) } if ! callgrind_annotate_candidate . status . success () { anyhow :: bail ! ("callgrind_annotate for candidate finished with an error (code = {:?})" , callgrind_annotate_candidate . status . code ()) } let string_base = String :: from_utf8 (callgrind_annotate_base . stdout) . context ("callgrind_annotate produced invalid UTF8") ? ; let string_candidate = String :: from_utf8 (callgrind_annotate_candidate . stdout) . context ("callgrind_annotate produced invalid UTF8") ? ; Ok (format ! ("Base output:\n{string_base}\n\
         =====\n\n\
         Candidate output:\n{string_candidate}\n")) }
};
}
