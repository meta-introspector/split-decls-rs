// Generated macro for render_const_scalar (function)
macro_rules! Depcrate_displayrender_const_scalar {
() => {
// Module: crate::display
// Provides: {"render_const_scalar"}
// Dependencies: {}
fn render_const_scalar < 'db > (f : & mut HirFormatter < '_ , 'db > , b : & [u8] , memory_map : & MemoryMap < 'db > , ty : Ty < 'db > ,) -> Result < () , HirDisplayError > { let trait_env = TraitEnvironment :: empty (f . krate ()) ; let infcx = f . interner . infer_ctxt () . build (TypingMode :: PostAnalysis) ; let ty = infcx . at (& ObligationCause :: new () , trait_env . env) . deeply_normalize (ty) . unwrap_or (ty) ; render_const_scalar_inner (f , b , memory_map , ty , trait_env) }
};
}
