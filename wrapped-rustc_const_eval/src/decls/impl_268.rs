macro_rules! deps {
    () => {
        ImmTy!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < Prov : Provenance > std :: fmt :: Debug for ImmTy < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("ImmTy") . field ("imm" , & self . imm) . field ("ty" , & format_args ! ("{}" , self . layout . ty)) . finish () } }
    };
}

impl_268!()