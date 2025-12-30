// Generated macro for missed_arguments (function)
macro_rules! Depcrate_errormissed_arguments {
() => {
// Module: crate::error
// Provides: {"missed_arguments"}
// Dependencies: {}
fn missed_arguments < 'a , I : MaybePat + Spanned + 'a > (test : & 'a ItemFn , args : impl Iterator < Item = & 'a I > + 'a ,) -> Errors < 'a > { Box :: new (args . filter_map (| it | it . maybe_pat () . map (| pat | (it , pat))) . filter (move | (_ , pat) | ! fn_args_has_pat (test , pat)) . map (| (missed , pat) | { syn :: Error :: new (missed . span () , format ! ("Missed argument: '{}' should be a test function argument." , pat . render_type ()) ,) }) ,) }
};
}
