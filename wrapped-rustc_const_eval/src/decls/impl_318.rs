macro_rules! deps {
    () => {
        LocalState!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl < Prov : Provenance > std :: fmt :: Debug for LocalState < '_ , Prov > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("LocalState") . field ("value" , & self . value) . field ("ty" , & self . layout . get () . map (| l | l . ty)) . finish () } }
    };
}

impl_318!()