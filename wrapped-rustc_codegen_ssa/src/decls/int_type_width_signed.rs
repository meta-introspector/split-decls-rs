macro_rules! int_type_width_signed {
    () => {
        fn int_type_width_signed (ty : Ty < '_ > , tcx : TyCtxt < '_ >) -> Option < (u64 , bool) > { match ty . kind () { ty :: Int (t) => { Some ((t . bit_width () . unwrap_or (u64 :: from (tcx . sess . target . pointer_width)) , true)) } ty :: Uint (t) => { Some ((t . bit_width () . unwrap_or (u64 :: from (tcx . sess . target . pointer_width)) , false)) } _ => None , } }
    };
}

int_type_width_signed!()