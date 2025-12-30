// Generated macro for is_expn_of (function)
macro_rules! Depcrateis_expn_of {
() => {
// Module: crate
// Provides: {"is_expn_of"}
// Dependencies: {}
# [doc = " Returns the pre-expansion span if this comes from an expansion of the"] # [doc = " macro `name`."] # [doc = " See also [`is_direct_expn_of`]."] # [must_use] pub fn is_expn_of (mut span : Span , name : Symbol) -> Option < Span > { loop { if span . from_expansion () { let data = span . ctxt () . outer_expn_data () ; let new_span = data . call_site ; if let ExpnKind :: Macro (MacroKind :: Bang , mac_name) = data . kind && mac_name == name { return Some (new_span) ; } span = new_span ; } else { return None ; } } }
};
}
