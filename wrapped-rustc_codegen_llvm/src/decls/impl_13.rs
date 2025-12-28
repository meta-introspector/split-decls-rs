macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl AbiBuilderMethods for Builder < '_ , '_ , '_ > { fn get_param (& mut self , index : usize) -> Self :: Value { llvm :: get_param (self . llfn () , index as c_uint) } }
    };
}

impl_13!()