// Generated macro for from_toc_id (function)
macro_rules! Depcrate_genericsfrom_toc_id {
() => {
// Module: crate::generics
// Provides: {"from_toc_id"}
// Dependencies: {}
fn from_toc_id < 'a > (it : & 'a Generics ,) -> impl Fn ((LocalTypeOrConstParamId , & 'a TypeOrConstParamData) ,) -> (GenericParamId , GenericParamDataRef < 'a >) { move | (local_id , p) : (_ , _) | { let id = TypeOrConstParamId { parent : it . def , local_id } ; match p { TypeOrConstParamData :: TypeParamData (p) => (GenericParamId :: TypeParamId (TypeParamId :: from_unchecked (id)) , GenericParamDataRef :: TypeParamData (p) ,) , TypeOrConstParamData :: ConstParamData (p) => (GenericParamId :: ConstParamId (ConstParamId :: from_unchecked (id)) , GenericParamDataRef :: ConstParamData (p) ,) , } } }
};
}
