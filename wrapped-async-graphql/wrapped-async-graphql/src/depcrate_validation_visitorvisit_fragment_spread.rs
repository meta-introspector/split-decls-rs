// Generated macro for visit_fragment_spread (function)
macro_rules! Depcrate_validation_visitorvisit_fragment_spread {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_fragment_spread"}
// Dependencies: {}
fn visit_fragment_spread < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , fragment_spread : & 'a Positioned < FragmentSpread > ,) { v . enter_fragment_spread (ctx , fragment_spread) ; visit_directives (v , ctx , & fragment_spread . node . directives) ; if v . mode () == VisitMode :: Inline { if let Some (fragment) = ctx . fragments . get (fragment_spread . node . fragment_name . node . as_str ()) { visit_selection_set (v , ctx , & fragment . node . selection_set) ; } } v . exit_fragment_spread (ctx , fragment_spread) ; }
};
}
