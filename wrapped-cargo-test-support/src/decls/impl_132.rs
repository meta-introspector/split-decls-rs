macro_rules! deps {
    () => {
        Request!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl fmt :: Debug for Request { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Request") . field ("url" , & self . url) . field ("method" , & self . method) . field ("authorization" , & self . authorization) . field ("if_modified_since" , & self . if_modified_since) . field ("if_none_match" , & self . if_none_match) . finish () } }
    };
}

impl_132!();