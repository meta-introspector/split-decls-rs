// Generated macro for impl_540 (impl)
macro_rules! Depcrate_proto_streams_streamsimpl_540 {
() => {
// Module: crate::proto::streams::streams
// Provides: {"impl_540"}
// Dependencies: {}
impl fmt :: Debug for OpaqueStreamRef { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { use std :: sync :: TryLockError :: * ; match self . inner . try_lock () { Ok (me) => { let stream = & me . store [self . key] ; fmt . debug_struct ("OpaqueStreamRef") . field ("stream_id" , & stream . id) . field ("ref_count" , & stream . ref_count) . finish () } Err (Poisoned (_)) => fmt . debug_struct ("OpaqueStreamRef") . field ("inner" , & "<Poisoned>") . finish () , Err (WouldBlock) => fmt . debug_struct ("OpaqueStreamRef") . field ("inner" , & "<Locked>") . finish () , } } }
};
}
