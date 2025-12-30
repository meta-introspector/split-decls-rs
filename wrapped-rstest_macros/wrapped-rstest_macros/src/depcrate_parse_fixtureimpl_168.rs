// Generated macro for impl_168 (impl)
macro_rules! Depcrate_parse_fixtureimpl_168 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_168"}
// Dependencies: {}
impl VisitMut for FixturesFunctionExtractor { fn visit_fn_arg_mut (& mut self , node : & mut FnArg) { let arg = match node . maybe_pat_type_mut () { Some (pt) => pt , None => return , } ; let (extracted , remain) : (Vec < _ > , Vec < _ >) = std :: mem :: take (& mut arg . attrs) . into_iter () . partition (| attr | attr_in (attr , & ["with" , "from"])) ; arg . attrs = remain ; let (pos , errors) = parse_attribute_args_just_once (extracted . iter () , "with") ; self . 1 . extend (errors) ; let (resolve , errors) : (Option < syn :: Path > , _) = parse_attribute_args_just_once (extracted . iter () , "from") ; self . 1 . extend (errors) ; match (resolve , arg . pat . maybe_ident ()) { (Some (res) , _) => self . 0 . push (Fixture :: new (arg . pat . as_ref () . clone () , res , pos . unwrap_or_default () ,)) , (None , Some (ident)) if pos . is_some () => self . 0 . push (Fixture :: new (arg . pat . as_ref () . clone () , ident . clone () . into () , pos . unwrap_or_default () ,)) , (None , None) if pos . is_some () => { self . 1 . push (syn :: Error :: new_spanned (node , crate :: error :: messages :: DESTRUCT_WITHOUT_FROM ,)) ; } _ => { } } } }
};
}
