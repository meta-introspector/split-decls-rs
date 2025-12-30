// Generated macro for impl_183 (impl)
macro_rules! Depcrate_channelimpl_183 {
() => {
// Module: crate::channel
// Provides: {"impl_183"}
// Dependencies: {}
impl < D , E : std :: fmt :: Debug > std :: fmt :: Debug for Channel < D , E > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Channel") . field ("rx_frame" , & self . rx_frame) . field ("rx_error" , & self . rx_error) . finish () } }
};
}
