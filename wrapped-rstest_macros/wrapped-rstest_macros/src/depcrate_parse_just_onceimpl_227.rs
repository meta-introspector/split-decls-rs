// Generated macro for impl_227 (impl)
macro_rules! Depcrate_parse_just_onceimpl_227 {
() => {
// Module: crate::parse::just_once
// Provides: {"impl_227"}
// Dependencies: {}
impl < B > VisitMut for JustOnceFnAttributeExtractor < '_ , B > where B : AttrBuilder < ItemFn > , B : Validator < ItemFn > , { fn visit_item_fn_mut (& mut self , item_fn : & mut ItemFn) { let attrs = std :: mem :: take (& mut item_fn . attrs) ; let (extracted , remain) : (Vec < _ > , Vec < _ >) = attrs . into_iter () . partition (| a | attr_is (a , self . name)) ; item_fn . attrs = remain ; let parsed = extracted . into_iter () . map (| attr | B :: build (attr . clone () , item_fn) . map (| t | (attr , t))) . collect :: < Result < Vec < _ > , _ > > () ; let mut errors = Vec :: default () ; let mut out = None ; match parsed { Ok (data) => match data . len () { 1 => match B :: validate (item_fn) { Ok (_) => { out = data . into_iter () . next () . map (| (_attr , t) | t) ; } Err (e) => { errors . push (e) ; } } , 0 => { } _ => { errors . extend (data . into_iter () . skip (1) . map (| (attr , _t) | { syn :: Error :: new_spanned (attr . into_token_stream () , format ! ("Cannot use #[{}] more than once." , self . name) ,) })) ; } } , Err (e) => { errors . push (e) ; } } ; if errors . is_empty () { self . inner = Ok (out) ; } else { self . inner = Err (errors) ; } } }
};
}
