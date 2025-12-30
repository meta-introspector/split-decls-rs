// Generated macro for impl_589 (impl)
macro_rules! Depcrate_displayimpl_589 {
() => {
// Module: crate::display
// Provides: {"impl_589"}
// Dependencies: {}
impl BoundsFormattingCtx { fn contains (& mut self , proj : & ProjectionTy) -> bool { match self { BoundsFormattingCtx :: Entered { projection_tys_met } => { projection_tys_met . contains (proj) } BoundsFormattingCtx :: Exited => false , } } }
};
}
