// Generated macro for impl_168 (impl)
macro_rules! Depcrate_strategy_hybridimpl_168 {
() => {
// Module: crate::strategy::hybrid
// Provides: {"impl_168"}
// Dependencies: {}
impl < T : RefCnt > Protected < T > for HybridProtection < T > { # [inline] fn from_inner (ptr : T) -> Self { Self { debt : None , ptr : ManuallyDrop :: new (ptr) , } } # [inline] fn into_inner (mut self) -> T { match self . debt . take () { None => () , Some (debt) => { let ptr = T :: inc (& self . ptr) ; if ! debt . pay :: < T > (ptr) { unsafe { T :: dec (ptr) } ; } } } let inner = unsafe { ptr :: read (self . ptr . deref ()) } ; mem :: forget (self) ; inner } }
};
}
