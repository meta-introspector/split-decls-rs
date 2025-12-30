// Generated macro for impl_770 (impl)
macro_rules! Depcrate_frameimpl_770 {
() => {
// Module: crate::frame
// Provides: {"impl_770"}
// Dependencies: {}
impl < T > Frame < T > { pub fn map < F , U > (self , f : F) -> Frame < U > where F : FnOnce (T) -> U , { use self :: Frame :: * ; match self { Data (frame) => frame . map (f) . into () , Headers (frame) => frame . into () , Priority (frame) => frame . into () , PushPromise (frame) => frame . into () , Settings (frame) => frame . into () , Ping (frame) => frame . into () , GoAway (frame) => frame . into () , WindowUpdate (frame) => frame . into () , Reset (frame) => frame . into () , } } }
};
}
