macro_rules! deps {
    () => {
        ModuleItemMap!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl fmt :: Debug for ModuleItemMap < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ModuleItemMap") . field ("module_id" , & self . module_id) . finish () } }
    };
}

impl_303!();