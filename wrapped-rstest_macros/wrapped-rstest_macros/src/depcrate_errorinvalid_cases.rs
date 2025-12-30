// Generated macro for invalid_cases (function)
macro_rules! Depcrate_errorinvalid_cases {
() => {
// Module: crate::error
// Provides: {"invalid_cases"}
// Dependencies: {}
fn invalid_cases (params : & RsTestData) -> Errors < '_ > { let n_args = params . case_args () . count () ; Box :: new (params . cases () . filter (move | case | case . args . len () != n_args) . map (| case | { syn :: Error :: new_spanned (case , "Wrong case signature: should match the given parameters list." ,) }) ,) }
};
}
