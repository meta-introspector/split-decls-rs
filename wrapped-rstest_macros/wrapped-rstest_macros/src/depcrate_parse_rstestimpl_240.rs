// Generated macro for impl_240 (impl)
macro_rules! Depcrate_parse_rstestimpl_240 {
() => {
// Module: crate::parse::rstest
// Provides: {"impl_240"}
// Dependencies: {}
impl Parse for RsTestInfo { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (if input . is_empty () { Default :: default () } else { Self { data : input . parse () ? , attributes : input . parse :: < Token ! [::] > () . or_else (| _ | Ok (Default :: default ())) . and_then (| _ | input . parse ()) ? , arguments : Default :: default () , } }) } }
};
}
