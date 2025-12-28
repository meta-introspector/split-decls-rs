macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'll , 'tcx > ArgAbiBuilderMethods < 'tcx > for Builder < '_ , 'll , 'tcx > { fn store_fn_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , idx : & mut usize , dst : PlaceRef < 'tcx , Self :: Value > ,) { arg_abi . store_fn_arg (self , idx , dst) } fn store_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , val : & 'll Value , dst : PlaceRef < 'tcx , & 'll Value > ,) { arg_abi . store (self , val , dst) } }
    };
}

impl_10!();