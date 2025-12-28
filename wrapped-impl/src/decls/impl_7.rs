macro_rules! deps {
    () => {
        Input!();
        Enum!();
        Struct!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > Input < 'a > { pub fn from_syn (node : & 'a DeriveInput) -> Result < Self > { match & node . data { Data :: Struct (data) => Struct :: from_syn (node , data) . map (Input :: Struct) , Data :: Enum (data) => Enum :: from_syn (node , data) . map (Input :: Enum) , Data :: Union (_) => Err (Error :: new_spanned (node , "union as errors are not supported" ,)) , } } }
    };
}

impl_7!()