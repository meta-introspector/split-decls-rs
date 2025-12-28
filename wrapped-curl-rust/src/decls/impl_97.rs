macro_rules! deps {
    () => {
        PostRedirections!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl fmt :: Debug for PostRedirections { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("PostRedirections") . field ("redirect_301" , & (self . bits & curl_sys :: CURL_REDIR_POST_301 != 0) ,) . field ("redirect_302" , & (self . bits & curl_sys :: CURL_REDIR_POST_302 != 0) ,) . field ("redirect_303" , & (self . bits & curl_sys :: CURL_REDIR_POST_303 != 0) ,) . finish () } }
    };
}

impl_97!();