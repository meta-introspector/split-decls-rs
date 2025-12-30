// Generated macro for impl_68 (impl)
macro_rules! Depcrate_parsingimpl_68 {
() => {
// Module: crate::parsing
// Provides: {"impl_68"}
// Dependencies: {}
impl RuleBuilder { fn try_add (& mut self , pattern : Result < SyntaxNode , () > , template : Option < Result < SyntaxNode , () > > ,) { match (pattern , template) { (Ok (pattern) , Some (Ok (template))) => self . rules . push (ParsedRule { placeholders_by_stand_in : self . placeholders_by_stand_in . clone () , pattern , template : Some (template) , }) , (Ok (pattern) , None) => self . rules . push (ParsedRule { placeholders_by_stand_in : self . placeholders_by_stand_in . clone () , pattern , template : None , }) , _ => { } } } fn build (mut self) -> Result < Vec < ParsedRule > , SsrError > { if self . rules . is_empty () { bail ! ("Not a valid Rust expression, type, item, path or pattern") ; } if self . rules . iter () . any (| rule | contains_path (& rule . pattern)) { let old_len = self . rules . len () ; self . rules . retain (| rule | contains_path (& rule . pattern)) ; if self . rules . len () < old_len { cov_mark :: hit ! (pattern_is_a_single_segment_path) ; } } Ok (self . rules) } }
};
}
