// Generated macro for impl_1038 (impl)
macro_rules! Depcrate_remote_callbacksimpl_1038 {
() => {
// Module: crate::remote_callbacks
// Provides: {"impl_1038"}
// Dependencies: {}
impl < 'a > Binding for RemoteCallbacks < 'a > { type Raw = raw :: git_remote_callbacks ; unsafe fn from_raw (_raw : raw :: git_remote_callbacks) -> RemoteCallbacks < 'a > { panic ! ("unimplemented") ; } fn raw (& self) -> raw :: git_remote_callbacks { unsafe { let mut callbacks : raw :: git_remote_callbacks = mem :: zeroed () ; assert_eq ! (raw :: git_remote_init_callbacks (& mut callbacks , raw :: GIT_REMOTE_CALLBACKS_VERSION) , 0) ; if self . progress . is_some () { callbacks . transfer_progress = Some (transfer_progress_cb) ; } if self . credentials . is_some () { callbacks . credentials = Some (credentials_cb) ; } if self . sideband_progress . is_some () { callbacks . sideband_progress = Some (sideband_progress_cb) ; } if self . certificate_check . is_some () { callbacks . certificate_check = Some (certificate_check_cb) ; } if self . push_update_reference . is_some () { callbacks . push_update_reference = Some (push_update_reference_cb) ; } if self . push_progress . is_some () { callbacks . push_transfer_progress = Some (push_transfer_progress_cb) ; } if self . pack_progress . is_some () { callbacks . pack_progress = Some (pack_progress_cb) ; } if self . update_tips . is_some () { let f : extern "C" fn (* const c_char , * const raw :: git_oid , * const raw :: git_oid , * mut c_void ,) -> c_int = update_tips_cb ; callbacks . update_tips = Some (f) ; } if self . push_negotiation . is_some () { callbacks . push_negotiation = Some (push_negotiation_cb) ; } callbacks . payload = self as * const _ as * mut _ ; callbacks } } }
};
}
