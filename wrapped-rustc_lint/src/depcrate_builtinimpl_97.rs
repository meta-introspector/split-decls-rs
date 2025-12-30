// Generated macro for impl_97 (impl)
macro_rules! Depcrate_builtinimpl_97 {
() => {
// Module: crate::builtin
// Provides: {"impl_97"}
// Dependencies: {}
impl EarlyLintPass for AnonymousParameters { fn check_trait_item (& mut self , cx : & EarlyContext < '_ > , it : & ast :: AssocItem) { if cx . sess () . edition () != Edition :: Edition2015 { return ; } if let ast :: AssocItemKind :: Fn (box Fn { ref sig , .. }) = it . kind { for arg in sig . decl . inputs . iter () { if let ast :: PatKind :: Missing = arg . pat . kind { let ty_snip = cx . sess () . source_map () . span_to_snippet (arg . ty . span) ; let (ty_snip , appl) = if let Ok (ref snip) = ty_snip { (snip . as_str () , Applicability :: MachineApplicable) } else { ("<type>" , Applicability :: HasPlaceholders) } ; cx . emit_span_lint (ANONYMOUS_PARAMETERS , arg . pat . span , BuiltinAnonymousParams { suggestion : (arg . pat . span , appl) , ty_snip } ,) ; } } } } }
};
}
