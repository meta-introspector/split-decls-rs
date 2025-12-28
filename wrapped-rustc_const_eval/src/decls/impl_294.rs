macro_rules! deps {
    () => {
        PlaceTy!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < Prov : Provenance > std :: fmt :: Debug for PlaceTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("PlaceTy") . field ("place" , & self . place) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
    };
}

impl_294!()