// Generated macro for is_direct_expn_of (function)
macro_rules! Depcrateis_direct_expn_of {
() => {
// Module: crate
// Provides: {"is_direct_expn_of"}
// Dependencies: {}
# [doc = " Returns the pre-expansion span if the span directly comes from an expansion"] # [doc = " of the macro `name`."] # [doc = " The difference with [`is_expn_of`] is that in"] # [doc = " ```no_run"] # [doc = " # macro_rules! foo { ($name:tt!$args:tt) => { $name!$args } }"] # [doc = " # macro_rules! bar { ($e:expr) => { $e } }"] # [doc = " foo!(bar!(42));"] # [doc = " ```"] # [doc = " `42` is considered expanded from `foo!` and `bar!` by `is_expn_of` but only"] # [doc = " from `bar!` by `is_direct_expn_of`."] # [must_use] pub fn is_direct_expn_of (span : Span , name : Symbol) -> Option < Span > { if span . from_expansion () { let data = span . ctxt () . outer_expn_data () ; let new_span = data . call_site ; if let ExpnKind :: Macro (MacroKind :: Bang , mac_name) = data . kind && mac_name == name { return Some (new_span) ; } } None }
};
}
