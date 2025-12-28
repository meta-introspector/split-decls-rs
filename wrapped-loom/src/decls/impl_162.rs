macro_rules! deps {
    () => {
        Thread!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl fmt :: Debug for Thread { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Thread") . field ("id" , & self . id) . field ("state" , & self . state) . field ("critical" , & self . critical) . field ("operation" , & self . operation) . field ("causality" , & self . causality) . field ("released" , & self . released) . field ("dpor_vv" , & self . dpor_vv) . field ("last_yield" , & self . last_yield) . field ("yield_count" , & self . yield_count) . field ("locals" , & format_args ! ("[..locals..]")) . finish () } }
    };
}

impl_162!();