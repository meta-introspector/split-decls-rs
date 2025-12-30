// Generated macro for impl_771 (impl)
macro_rules! Depcrate_frameimpl_771 {
() => {
// Module: crate::frame
// Provides: {"impl_771"}
// Dependencies: {}
impl < T > fmt :: Debug for Frame < T > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { use self :: Frame :: * ; match * self { Data (ref frame) => fmt :: Debug :: fmt (frame , fmt) , Headers (ref frame) => fmt :: Debug :: fmt (frame , fmt) , Priority (ref frame) => fmt :: Debug :: fmt (frame , fmt) , PushPromise (ref frame) => fmt :: Debug :: fmt (frame , fmt) , Settings (ref frame) => fmt :: Debug :: fmt (frame , fmt) , Ping (ref frame) => fmt :: Debug :: fmt (frame , fmt) , GoAway (ref frame) => fmt :: Debug :: fmt (frame , fmt) , WindowUpdate (ref frame) => fmt :: Debug :: fmt (frame , fmt) , Reset (ref frame) => fmt :: Debug :: fmt (frame , fmt) , } } }
};
}
