// Generated macro for put_error_inner (function)
macro_rules! Depcrateput_error_inner {
() => {
// Module: crate
// Provides: {"put_error_inner"}
// Dependencies: {}
unsafe fn put_error_inner (library : c_int , func : FunctionInner , reason : c_int , file : & 'static str , line : u32 , message : Option < Cow < 'static , str > > ,) { cfg_if ! { if # [cfg (ossl300)] { openssl_sys :: ERR_new () ; openssl_sys :: ERR_set_debug (file . as_ptr () as * const c_char , line as c_int , func ,) ; openssl_sys :: ERR_set_error (library , reason , ptr :: null ()) ; } else { openssl_sys :: ERR_put_error (library , func , reason , file . as_ptr () as * const c_char , line as c_int ,) ; } } let data = match message { Some (Cow :: Borrowed (s)) => Some ((s . as_ptr () as * const c_char as * mut c_char , 0)) , Some (Cow :: Owned (s)) => { let ptr = openssl_sys :: CRYPTO_malloc (s . len () as _ , concat ! (file ! () , "\0") . as_ptr () as * const c_char , line ! () as c_int ,) as * mut c_char ; if ptr . is_null () { None } else { ptr :: copy_nonoverlapping (s . as_ptr () , ptr as * mut u8 , s . len ()) ; Some ((ptr , openssl_sys :: ERR_TXT_MALLOCED)) } } None => None , } ; if let Some ((ptr , flags)) = data { openssl_sys :: ERR_set_error_data (ptr , flags | openssl_sys :: ERR_TXT_STRING) ; } }
};
}
