// Generated macro for check_param_inner (function)
macro_rules! Depcrate_needless_arbitrary_self_typecheck_param_inner {
() => {
// Module: crate::needless_arbitrary_self_type
// Provides: {"check_param_inner"}
// Dependencies: {}
fn check_param_inner (cx : & EarlyContext < '_ > , path : & Path , span : Span , binding_mode : & Mode , mutbl : Mutability) { if let [segment] = & path . segments [..] && segment . ident . name == kw :: SelfUpper { let mut applicability = Applicability :: MachineApplicable ; let self_param = match (binding_mode , mutbl) { (Mode :: Ref (None) , Mutability :: Mut) => "&mut self" . to_string () , (Mode :: Ref (Some (lifetime)) , Mutability :: Mut) => { if lifetime . ident . span . from_expansion () { applicability = Applicability :: HasPlaceholders ; "&'_ mut self" . to_string () } else { let lt_name = snippet_with_applicability (cx , lifetime . ident . span , ".." , & mut applicability) ; format ! ("&{lt_name} mut self") } } , (Mode :: Ref (None) , Mutability :: Not) => "&self" . to_string () , (Mode :: Ref (Some (lifetime)) , Mutability :: Not) => { if lifetime . ident . span . from_expansion () { applicability = Applicability :: HasPlaceholders ; "&'_ self" . to_string () } else { let lt_name = snippet_with_applicability (cx , lifetime . ident . span , ".." , & mut applicability) ; format ! ("&{lt_name} self") } } , (Mode :: Value , Mutability :: Mut) => "mut self" . to_string () , (Mode :: Value , Mutability :: Not) => "self" . to_string () , } ; span_lint_and_sugg (cx , NEEDLESS_ARBITRARY_SELF_TYPE , span , "the type of the `self` parameter does not need to be arbitrary" , "consider to change this parameter to" , self_param , applicability ,) ; } }
};
}
