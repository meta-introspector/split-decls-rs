macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! cvt {
    () => {
        deps!();
        fn cvt (r : curl_sys :: CURLcode) -> Result < () , Error > { if r == curl_sys :: CURLE_OK { Ok (()) } else { Err (Error :: new (r)) } }
    };
}

cvt!();