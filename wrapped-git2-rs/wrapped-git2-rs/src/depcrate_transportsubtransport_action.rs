// Generated macro for subtransport_action (function)
macro_rules! Depcrate_transportsubtransport_action {
() => {
// Module: crate::transport
// Provides: {"subtransport_action"}
// Dependencies: {}
extern "C" fn subtransport_action (stream : * mut * mut raw :: git_smart_subtransport_stream , raw_transport : * mut raw :: git_smart_subtransport , url : * const c_char , action : raw :: git_smart_service_t ,) -> c_int { panic :: wrap (| | unsafe { let url = CStr :: from_ptr (url) . to_bytes () ; let url = match str :: from_utf8 (url) . ok () { Some (s) => s , None => return - 1 , } ; let action = match action { raw :: GIT_SERVICE_UPLOADPACK_LS => Service :: UploadPackLs , raw :: GIT_SERVICE_UPLOADPACK => Service :: UploadPack , raw :: GIT_SERVICE_RECEIVEPACK_LS => Service :: ReceivePackLs , raw :: GIT_SERVICE_RECEIVEPACK => Service :: ReceivePack , n => panic ! ("unknown action: {}" , n) , } ; let transport = & mut * (raw_transport as * mut RawSmartSubtransport) ; let generate_stream = transport . rpc || action == Service :: UploadPackLs || action == Service :: ReceivePackLs ; if generate_stream { let obj = match transport . obj . action (url , action) { Ok (s) => s , Err (e) => return e . raw_set_git_error () , } ; * stream = mem :: transmute (Box :: new (RawSmartSubtransportStream { raw : raw :: git_smart_subtransport_stream { subtransport : raw_transport , read : Some (stream_read) , write : Some (stream_write) , free : Some (stream_free) , } , obj , })) ; transport . stream = Some (* stream) ; } else { if transport . stream . is_none () { return - 1 ; } * stream = transport . stream . unwrap () ; } 0 }) . unwrap_or (- 1) }
};
}
