macro_rules! deps {
    () => {
        Handler!();
        Inner!();
        InfoType!();
    };
}

macro_rules! debug_cb {
    () => {
        deps!();
        extern "C" fn debug_cb < H : Handler > (_handle : * mut curl_sys :: CURL , kind : curl_sys :: curl_infotype , data : * mut c_char , size : size_t , userptr : * mut c_void ,) -> c_int { panic :: catch (| | unsafe { let data = slice :: from_raw_parts (data as * const u8 , size) ; let kind = match kind { curl_sys :: CURLINFO_TEXT => InfoType :: Text , curl_sys :: CURLINFO_HEADER_IN => InfoType :: HeaderIn , curl_sys :: CURLINFO_HEADER_OUT => InfoType :: HeaderOut , curl_sys :: CURLINFO_DATA_IN => InfoType :: DataIn , curl_sys :: CURLINFO_DATA_OUT => InfoType :: DataOut , curl_sys :: CURLINFO_SSL_DATA_IN => InfoType :: SslDataIn , curl_sys :: CURLINFO_SSL_DATA_OUT => InfoType :: SslDataOut , _ => return , } ; (* (userptr as * mut Inner < H >)) . handler . debug (kind , data) }) ; 0 }
    };
}

debug_cb!();