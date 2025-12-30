// Generated macro for impl_399 (impl)
macro_rules! Depcrate_render_testimpl_399 {
() => {
// Module: crate::render::test
// Provides: {"impl_399"}
// Dependencies: {}
impl < 'ast > Visit < 'ast > for Assignments { fn visit_local (& mut self , assign : & syn :: Local) { match & assign { syn :: Local { pat : syn :: Pat :: Ident (pat) , init : Some (LocalInit { expr , .. }) , .. } => { self . 0 . insert (pat . ident . to_string () , expr . as_ref () . clone ()) ; } _ => { } } } }
};
}
