// Generated macro for impl_629 (impl)
macro_rules! Depcrate_displayimpl_629 {
() => {
// Module: crate::display
// Provides: {"impl_629"}
// Dependencies: {}
impl HirDisplay for DomainGoal { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { match self { DomainGoal :: Holds (wc) => { write ! (f , "Holds(") ? ; wc . hir_fmt (f) ? ; write ! (f , ")") ? ; } _ => write ! (f , "_") ? , } Ok (()) } }
};
}
