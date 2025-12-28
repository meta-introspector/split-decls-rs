macro_rules! cvt {
    () => {
        fn cvt (r : curl_sys :: CURLcode) -> Result < () , Error > { if r == curl_sys :: CURLE_OK { Ok (()) } else { Err (Error :: new (r)) } }
    };
}

cvt!()