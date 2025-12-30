// Generated macro for ExpansionInfo (struct)
macro_rules! DepcrateExpansionInfo {
() => {
// Module: crate
// Provides: {"ExpansionInfo"}
// Dependencies: {}
# [doc = " ExpansionInfo mainly describes how to map text range between src and expanded macro"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ExpansionInfo { expanded : InMacroFile < SyntaxNode > , # [doc = " The argument TokenTree or item for attributes"] arg : InFile < Option < SyntaxNode > > , exp_map : Arc < ExpansionSpanMap > , arg_map : SpanMap , loc : MacroCallLoc , }
};
}
