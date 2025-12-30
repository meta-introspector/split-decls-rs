// Generated macro for impl_38 (impl)
macro_rules! Depcrate_gen_implimpl_38 {
() => {
// Module: crate::gen_impl
// Provides: {"impl_38"}
// Dependencies: {}
impl < A , T , const LOCAL : bool > fmt :: Debug for GeneratorObj < '_ , A , T , LOCAL > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Generator<{}, Output={}, Local={}> {{ ... }}" , std :: any :: type_name ::< A > () , std :: any :: type_name ::< T > () , LOCAL) } }
};
}
