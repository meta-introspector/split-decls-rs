macro_rules! deps {
    () => {
        ContextError!();
        Error!();
        Quoted!();
        Result!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < C , E > Debug for ContextError < C , E > where C : Display , E : Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Error") . field ("context" , & Quoted (& self . context)) . field ("source" , & self . error) . finish () } }
    };
}

impl_25!()