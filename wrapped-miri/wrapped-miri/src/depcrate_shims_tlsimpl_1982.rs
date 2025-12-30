// Generated macro for impl_1982 (impl)
macro_rules! Depcrate_shims_tlsimpl_1982 {
() => {
// Module: crate::shims::tls
// Provides: {"impl_1982"}
// Dependencies: {}
impl < 'tcx > TlsDtorsState < 'tcx > { pub fn on_stack_empty (& mut self , this : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , Poll < () > > { use TlsDtorsStatePriv :: * ; let new_state = 'new_state : { match & mut self . 0 { Init => { match this . tcx . sess . target . os { Os :: MacOs => { break 'new_state MacOsDtors ; } _ if this . target_os_is_unix () => { break 'new_state PthreadDtors (Default :: default ()) ; } Os :: Windows => { let dtors = this . lookup_windows_tls_dtors () ? ; break 'new_state WindowsDtors (dtors) ; } _ => { break 'new_state Done ; } } } MacOsDtors => { match this . schedule_macos_tls_dtor () ? { Poll :: Pending => return interp_ok (Poll :: Pending) , Poll :: Ready (()) => break 'new_state PthreadDtors (Default :: default ()) , } } PthreadDtors (state) => { match this . schedule_next_pthread_tls_dtor (state) ? { Poll :: Pending => return interp_ok (Poll :: Pending) , Poll :: Ready (()) => break 'new_state Done , } } WindowsDtors (dtors) => { if let Some (dtor) = dtors . pop () { this . schedule_windows_tls_dtor (dtor) ? ; return interp_ok (Poll :: Pending) ; } else { break 'new_state Done ; } } Done => { this . machine . tls . delete_all_thread_tls (this . active_thread ()) ; return interp_ok (Poll :: Ready (())) ; } } } ; self . 0 = new_state ; interp_ok (Poll :: Pending) } }
};
}
