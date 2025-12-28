macro_rules! deps {
    () => {
        ReadError!();
        Inner!();
        Handler!();
    };
}

macro_rules! read_cb {
    () => {
        deps!();
        extern "C" fn read_cb < H : Handler > (ptr : * mut c_char , size : size_t , nmemb : size_t , data : * mut c_void ,) -> size_t { panic :: catch (| | unsafe { let input = slice :: from_raw_parts_mut (ptr as * mut u8 , size * nmemb) ; match (* (data as * mut Inner < H >)) . handler . read (input) { Ok (s) => s , Err (ReadError :: Pause) => curl_sys :: CURL_READFUNC_PAUSE , Err (ReadError :: Abort) => curl_sys :: CURL_READFUNC_ABORT , } }) . unwrap_or (! 0) }
    };
}

read_cb!();