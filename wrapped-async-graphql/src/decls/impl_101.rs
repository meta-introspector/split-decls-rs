macro_rules! deps {
    () => {
        Result!();
        Request!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl Debug for Request { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { f . debug_struct ("Request") . field ("query" , & self . query) . field ("operation_name" , & self . operation_name) . field ("variables" , & self . variables) . field ("extensions" , & self . extensions) . finish () } }
    };
}

impl_101!()