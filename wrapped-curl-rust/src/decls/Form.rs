macro_rules! deps {
    () => {
        List!();
    };
}

macro_rules! Form {
    () => {
        deps!();
        # [doc = " Multipart/formdata for an HTTP POST request."] # [doc = ""] # [doc = " This structure is built up and then passed to the `Easy::httppost` method to"] # [doc = " be sent off with a request."] pub struct Form { head : * mut curl_sys :: curl_httppost , tail : * mut curl_sys :: curl_httppost , headers : Vec < List > , buffers : Vec < Vec < u8 > > , strings : Vec < CString > , }
    };
}

Form!()