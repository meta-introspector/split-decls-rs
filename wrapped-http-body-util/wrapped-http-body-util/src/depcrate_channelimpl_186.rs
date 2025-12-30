// Generated macro for impl_186 (impl)
macro_rules! Depcrate_channelimpl_186 {
() => {
// Module: crate::channel
// Provides: {"impl_186"}
// Dependencies: {}
impl < D , E : std :: fmt :: Debug > std :: fmt :: Debug for Sender < D , E > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Sender") . field ("tx_frame" , & self . tx_frame) . field ("tx_error" , & self . tx_error) . finish () } }
};
}
