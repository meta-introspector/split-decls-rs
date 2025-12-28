macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! memset_intrinsic {
    () => {
        deps!();
        fn memset_intrinsic < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , volatile : bool , ty : Ty < 'tcx > , dst : Bx :: Value , val : Bx :: Value , count : Bx :: Value ,) { let layout = bx . layout_of (ty) ; let size = layout . size ; let align = layout . align . abi ; let size = bx . mul (bx . const_usize (size . bytes ()) , count) ; let flags = if volatile { MemFlags :: VOLATILE } else { MemFlags :: empty () } ; bx . memset (dst , val , size , align , flags) ; }
    };
}

memset_intrinsic!()