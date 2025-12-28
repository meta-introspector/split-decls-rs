macro_rules! deps {
    () => {
        Editor!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl std :: fmt :: Debug for Editor < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Editor") . field ("object_hash" , & self . object_hash) . field ("path_buf" , & self . path_buf) . field ("trees" , & self . trees) . finish () } }
    };
}

impl_103!();