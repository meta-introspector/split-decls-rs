macro_rules! deps {
    () => {
        BackendTypes!();
        PlaceRef!();
    };
}

macro_rules! ArgAbiBuilderMethods {
    () => {
        deps!();
        pub trait ArgAbiBuilderMethods < 'tcx > : BackendTypes { fn store_fn_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , idx : & mut usize , dst : PlaceRef < 'tcx , Self :: Value > ,) ; fn store_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , val : Self :: Value , dst : PlaceRef < 'tcx , Self :: Value > ,) ; }
    };
}

ArgAbiBuilderMethods!()