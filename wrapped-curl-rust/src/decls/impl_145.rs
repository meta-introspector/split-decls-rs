macro_rules! deps {
    () => {
        Events!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl fmt :: Debug for Events { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Events") . field ("input" , & (self . bits & curl_sys :: CURL_CSELECT_IN != 0)) . field ("output" , & (self . bits & curl_sys :: CURL_CSELECT_OUT != 0)) . field ("error" , & (self . bits & curl_sys :: CURL_CSELECT_ERR != 0)) . finish () } }
    };
}

impl_145!();