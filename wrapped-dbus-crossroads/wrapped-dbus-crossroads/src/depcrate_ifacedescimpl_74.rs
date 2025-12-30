// Generated macro for impl_74 (impl)
macro_rules! Depcrate_ifacedescimpl_74 {
() => {
// Module: crate::ifacedesc
// Provides: {"impl_74"}
// Dependencies: {}
impl < T : std :: marker :: Send , A > PropBuilder < '_ , T , A > { pub fn annotate < N : Into < String > , V : Into < String > > (self , name : N , value : V) -> Self { self . desc . annotations . insert (name , value) ; self } pub fn deprecated (self) -> Self { self . annotate (DEPRECATED , "true") } pub fn emits_changed_false (mut self) -> Self { self . emits_changed = EmitsChangedSignal :: False ; self . annotate (EMITS_CHANGED , "false") } # [doc = " This means that the property never changes. Attempts to change it or make an \"PropertiesChanged\""] # [doc = " signal will result in a panic."] pub fn emits_changed_const (mut self) -> Self { self . emits_changed = EmitsChangedSignal :: Const ; self . annotate (EMITS_CHANGED , "const") } pub fn emits_changed_invalidates (mut self) -> Self { self . emits_changed = EmitsChangedSignal :: Invalidates ; self . annotate (EMITS_CHANGED , "invalidates") } pub fn emits_changed_true (mut self) -> Self { self . emits_changed = EmitsChangedSignal :: True ; self . annotate (EMITS_CHANGED , "true") } }
};
}
