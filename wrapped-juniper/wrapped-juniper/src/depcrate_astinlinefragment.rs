// Generated macro for InlineFragment (struct)
macro_rules! Depcrate_astInlineFragment {
() => {
// Module: crate::ast
// Provides: {"InlineFragment"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq)] pub struct InlineFragment < 'a , S > { pub type_condition : Option < Spanning < & 'a str > > , pub directives : Option < Vec < Spanning < Directive < 'a , S > > > > , pub selection_set : Vec < Selection < 'a , S > > , }
};
}
