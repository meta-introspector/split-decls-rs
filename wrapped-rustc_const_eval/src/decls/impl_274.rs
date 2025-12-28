macro_rules! deps {
    () => {
        OpTy!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < Prov : Provenance > std :: fmt :: Debug for OpTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("OpTy") . field ("op" , & self . op) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
    };
}

impl_274!();