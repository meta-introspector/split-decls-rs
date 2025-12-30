// Generated macro for clone_resetable (macro)
macro_rules! Depcrate_streamclone_resetable {
() => {
// Module: crate::stream
// Provides: {"clone_resetable"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! clone_resetable { (($ ($ params : tt) *) $ ty : ty) => { impl <$ ($ params) *> ResetStream for $ ty where Self : StreamOnce { type Checkpoint = Self ; fn checkpoint (& self) -> Self { self . clone () } # [inline] fn reset (& mut self , checkpoint : Self) -> Result < () , Self :: Error > { * self = checkpoint ; Ok (()) } } } }
};
}
