// Generated macro for impl_425 (impl)
macro_rules! Depcrate_sessionimpl_425 {
() => {
// Module: crate::session
// Provides: {"impl_425"}
// Dependencies: {}
impl SessionStoreBroker { pub fn new (get_cb : SessionStoreGetCallback , put_cb : SessionStorePutCallback) -> Self { SessionStoreBroker { get_cb , put_cb } } fn retrieve (& self , key : & [u8] , remove : bool) -> Option < Vec < u8 > > { let key : rustls_slice_bytes = key . into () ; let userdata = userdata_get () . ok () ? ; let mut data = vec ! [0 ; 65 * 1024] ; let mut out_n = 0 ; let cb = self . get_cb ; let result = unsafe { cb (userdata , & key , remove as c_int , data . as_mut_ptr () , data . len () , & mut out_n ,) } ; match rustls_result :: from (result) { rustls_result :: Ok => { unsafe { data . set_len (out_n) } ; Some (data) } _ => None , } } fn store (& self , key : Vec < u8 > , value : Vec < u8 >) -> bool { let key = key . as_slice () . into () ; let value = value . as_slice () . into () ; let cb = self . put_cb ; let userdata = match userdata_get () { Ok (u) => u , Err (_) => return false , } ; let result = unsafe { cb (userdata , & key , & value) } ; result == rustls_result :: Ok as u32 } }
};
}
