// Generated macro for impl_12 (impl)
macro_rules! Depcrate_validatorimpl_12 {
() => {
// Module: crate::validator
// Provides: {"impl_12"}
// Dependencies: {}
impl fmt :: Display for EntityIdentifier { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match (& self . config , & self . split) { (Some (config) , Some (split)) => write ! (f , "{}/{}/{}" , self . dataset , config , split) , (Some (config) , None) => write ! (f , "{}/{}" , self . dataset , config) , _ => write ! (f , "{}" , self . dataset) , } } }
};
}
