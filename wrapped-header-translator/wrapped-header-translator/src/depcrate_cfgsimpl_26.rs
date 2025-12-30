// Generated macro for impl_26 (impl)
macro_rules! Depcrate_cfgsimpl_26 {
() => {
// Module: crate::cfgs
// Provides: {"impl_26"}
// Dependencies: {}
impl CfgState { fn new (enabled : bool) -> Self { if enabled { Self :: ShouldGate } else { Self :: AlreadyGated } } fn explicit (should_gate : bool) -> Self { if should_gate { Self :: ShouldGate } else { Self :: Omit } } fn dependency (& mut self , dependency : bool) { * self = match (* self , dependency) { (Self :: ShouldGate , true) => Self :: ShouldGate , (Self :: ShouldGate , false) => Self :: Omit , (Self :: AlreadyGated , _) => Self :: AlreadyGated , (Self :: Omit , _) => Self :: Omit , } ; } fn implied (& mut self , implied : bool) { * self = match (* self , implied) { (Self :: ShouldGate , false) => Self :: ShouldGate , (_ , false) => Self :: AlreadyGated , (state , true) => state , } ; } fn active (& self) -> bool { matches ! (self , Self :: ShouldGate) } fn allowed_active (& self) -> bool { matches ! (self , Self :: ShouldGate | Self :: AlreadyGated) } }
};
}
