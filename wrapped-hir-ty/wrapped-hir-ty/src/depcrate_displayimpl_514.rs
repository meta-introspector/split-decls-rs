// Generated macro for impl_514 (impl)
macro_rules! Depcrate_displayimpl_514 {
() => {
// Module: crate::display
// Provides: {"impl_514"}
// Dependencies: {}
impl < 'db > BoundsFormattingCtx < 'db > { fn contains (& self , proj : & AliasTy < 'db >) -> bool { match self { BoundsFormattingCtx :: Entered { projection_tys_met } => { projection_tys_met . contains (proj) } BoundsFormattingCtx :: Exited => false , } } }
};
}
