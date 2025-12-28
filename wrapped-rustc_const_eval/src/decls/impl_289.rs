macro_rules! deps {
    () => {
        MPlaceTy!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < Prov : Provenance > std :: fmt :: Debug for MPlaceTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("MPlaceTy") . field ("mplace" , & self . mplace) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
    };
}

impl_289!()