macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! ArgAbiExt {
    () => {
        deps!();
        trait ArgAbiExt < 'll , 'tcx > { fn store (& self , bx : & mut Builder < '_ , 'll , 'tcx > , val : & 'll Value , dst : PlaceRef < 'tcx , & 'll Value > ,) ; fn store_fn_arg (& self , bx : & mut Builder < '_ , 'll , 'tcx > , idx : & mut usize , dst : PlaceRef < 'tcx , & 'll Value > ,) ; }
    };
}

ArgAbiExt!()