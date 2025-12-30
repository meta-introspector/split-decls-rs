// Generated macro for cases_data (function)
macro_rules! Depcrate_rendercases_data {
() => {
// Module: crate::render
// Provides: {"cases_data"}
// Dependencies: {}
fn cases_data (info : & RsTestInfo , name_span : Span) -> impl Iterator < Item = CaseDataValues < '_ > > { let display_len = info . data . cases () . count () . display_len () ; info . data . cases () . enumerate () . map ({ move | (n , case) | { let resolver_case = info . data . case_args () . cloned () . map (| arg | info . arguments . inner_pat (& arg) . clone ()) . zip (case . args . iter ()) . collect :: < HashMap < _ , _ > > () ; CaseDataValues :: new (Ident :: new (& format_case_name (case , n + 1 , display_len) , name_span) , case . attrs . as_slice () , Box :: new (resolver_case) , Some (CaseInfo :: new (case . description . clone () , n)) ,) } }) }
};
}
