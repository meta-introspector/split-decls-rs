// Generated macro for impl_278 (impl)
macro_rules! Depcrate_parse_vlistimpl_278 {
() => {
// Module: crate::parse::vlist
// Provides: {"impl_278"}
// Dependencies: {}
impl Parse for ValueList { fn parse (input : ParseStream) -> Result < Self > { let ident : Ident = input . parse () ? ; let _to : Token ! [=>] = input . parse () ? ; let content ; let paren = syn :: bracketed ! (content in input) ; let values : Expressions = content . parse () ? ; let ret = Self { arg : ident . into_pat () , values : values . take () . into_iter () . map (| e | e . into ()) . collect () , } ; if ret . values . is_empty () { Err (syn :: Error :: new (paren . span . join () , "Values list should not be empty" ,)) } else { Ok (ret) } } }
};
}
