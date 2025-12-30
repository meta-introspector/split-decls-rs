// Generated macro for impl_303 (impl)
macro_rules! Depcrate_parseimpl_303 {
() => {
// Module: crate::parse
// Provides: {"impl_303"}
// Dependencies: {}
impl VisitMut for PartialsTypeFunctionExtractor { fn visit_item_fn_mut (& mut self , node : & mut ItemFn) { let attrs = std :: mem :: take (& mut node . attrs) ; let (partials , remain) : (Vec < _ > , Vec < _ >) = attrs . into_iter () . partition (| attr | match attr . path () . get_ident () { Some (name) => name . to_string () . starts_with (FixtureModifiers :: PARTIAL_RET_ATTR) , None => false , }) ; node . attrs = remain ; let mut errors = ErrorsVec :: default () ; let mut data : Vec < (usize , syn :: Type) > = Vec :: default () ; for attr in partials { match attr . parse_args :: < syn :: Type > () { Ok (t) => { match attr . path () . get_ident () . unwrap () . to_string () [FixtureModifiers :: PARTIAL_RET_ATTR . len () ..] . parse () { Ok (id) => data . push ((id , t)) , Err (_) => errors . push (syn :: Error :: new_spanned (attr , "Invalid partial syntax: should be partial_<n_arguments>" ,)) , } } Err (e) => errors . push (e) , } } self . 0 = if ! errors . is_empty () { Err (errors) } else { Ok (data) } ; } }
};
}
