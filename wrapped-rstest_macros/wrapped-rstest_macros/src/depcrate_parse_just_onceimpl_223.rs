// Generated macro for impl_223 (impl)
macro_rules! Depcrate_parse_just_onceimpl_223 {
() => {
// Module: crate::parse::just_once
// Provides: {"impl_223"}
// Dependencies: {}
impl < B > VisitMut for JustOnceFnArgAttributeExtractor < '_ , B > where B : AttrBuilder < Pat > , B : Validator < FnArg > , { fn visit_fn_arg_mut (& mut self , node : & mut FnArg) { let pat = match node . maybe_pat () { Some (pat) => pat . clone () , None => return , } ; if let FnArg :: Typed (ref mut arg) = node { let attrs = std :: mem :: take (& mut arg . attrs) ; let (extracted , remain) : (Vec < _ > , Vec < _ >) = attrs . into_iter () . partition (| a | attr_is (a , self . name)) ; arg . attrs = remain ; let parsed = extracted . into_iter () . map (| attr | B :: build (attr . clone () , & pat) . map (| t | (attr , t))) . collect :: < Result < Vec < _ > , _ > > () ; match parsed { Ok (data) => match data . len () { 1 => match B :: validate (node) { Ok (_) => self . elements . extend (data . into_iter () . map (| (_attr , t) | t)) , Err (e) => { self . errors . push (e) ; } } , 0 => { } _ => { self . errors . extend (data . into_iter () . skip (1) . map (| (attr , _t) | { syn :: Error :: new_spanned (attr . into_token_stream () , format ! ("Cannot use #[{}] more than once." , self . name) ,) })) ; } } , Err (e) => { self . errors . push (e) ; } } } } }
};
}
