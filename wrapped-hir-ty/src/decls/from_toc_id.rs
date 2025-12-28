macro_rules! deps {
    () => {
        Generics!();
    };
}

macro_rules! from_toc_id {
    () => {
        deps!();
        fn from_toc_id < 'a > (it : & 'a Generics ,) -> impl Fn ((LocalTypeOrConstParamId , & 'a TypeOrConstParamData) ,) -> (GenericParamId , GenericParamDataRef < 'a >) { move | (local_id , p) : (_ , _) | { let id = TypeOrConstParamId { parent : it . def , local_id } ; match p { TypeOrConstParamData :: TypeParamData (p) => (GenericParamId :: TypeParamId (TypeParamId :: from_unchecked (id)) , GenericParamDataRef :: TypeParamData (p) ,) , TypeOrConstParamData :: ConstParamData (p) => (GenericParamId :: ConstParamId (ConstParamId :: from_unchecked (id)) , GenericParamDataRef :: ConstParamData (p) ,) , } } }
    };
}

from_toc_id!()