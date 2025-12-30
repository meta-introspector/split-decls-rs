// Generated macro for impl_205 (impl)
macro_rules! Depcrate_dictionaryimpl_205 {
() => {
// Module: crate::dictionary
// Provides: {"impl_205"}
// Dependencies: {}
impl < KeyType : fmt :: Debug + Message , ObjectType : fmt :: Debug + Message > fmt :: Debug for NSDictionary < KeyType , ObjectType > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (keys , objects) = unsafe { self . to_vecs_unchecked () } ; let iter = keys . into_iter () . zip (objects) ; f . debug_map () . entries (iter) . finish () } }
};
}
