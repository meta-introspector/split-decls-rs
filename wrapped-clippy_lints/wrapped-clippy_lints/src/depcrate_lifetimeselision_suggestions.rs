// Generated macro for elision_suggestions (function)
macro_rules! Depcrate_lifetimeselision_suggestions {
() => {
// Module: crate::lifetimes
// Provides: {"elision_suggestions"}
// Dependencies: {}
fn elision_suggestions (cx : & LateContext < '_ > , generics : & Generics < '_ > , elidable_lts : & [LocalDefId] , usages : & [ElidableUsage] ,) -> Option < Vec < (Span , String) > > { let explicit_params = generics . params . iter () . filter (| param | ! param . is_elided_lifetime () && ! param . is_impl_trait ()) . collect :: < Vec < _ > > () ; let mut suggestions = if elidable_lts . len () == explicit_params . len () { vec ! [(generics . span , String :: new ())] } else { let mut end : Option < LocalDefId > = None ; elidable_lts . iter () . rev () . map (| & id | { let (idx , param) = explicit_params . iter () . find_position (| param | param . def_id == id) ? ; let span = if let Some (next) = explicit_params . get (idx + 1) && end != Some (next . def_id) { param . span . until (next . span) } else { end = Some (param . def_id) ; let prev = explicit_params . get (idx - 1) ? ; param . span . with_lo (prev . span . hi ()) } ; Some ((span , String :: new ())) }) . collect :: < Option < Vec < _ > > > () ? } ; suggestions . extend (usages . iter () . map (| & usage | { match usage { ElidableUsage :: Ref (span) => { let span = cx . sess () . source_map () . span_extend_while_whitespace (span) ; (span , String :: new ()) } , ElidableUsage :: Other (span) => { (span , String :: from ("'_")) } , } })) ; Some (suggestions) }
};
}
