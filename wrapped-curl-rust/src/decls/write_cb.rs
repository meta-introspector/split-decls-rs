macro_rules! deps {
    () => {
        Inner!();
        Handler!();
        WriteError!();
    };
}

macro_rules! write_cb {
    () => {
        deps!();
        extern "C" fn write_cb < H : Handler > (ptr : * mut c_char , size : size_t , nmemb : size_t , data : * mut c_void ,) -> size_t { panic :: catch (| | unsafe { let input = slice :: from_raw_parts (ptr as * const u8 , size * nmemb) ; match (* (data as * mut Inner < H >)) . handler . write (input) { Ok (s) => s , Err (WriteError :: Pause) => curl_sys :: CURL_WRITEFUNC_PAUSE , } }) . unwrap_or (! 0) }
    };
}

write_cb!()