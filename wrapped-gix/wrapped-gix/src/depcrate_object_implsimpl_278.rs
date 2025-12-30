// Generated macro for impl_278 (impl)
macro_rules! Depcrate_object_implsimpl_278 {
() => {
// Module: crate::object::impls
// Provides: {"impl_278"}
// Dependencies: {}
impl std :: fmt :: Debug for Object < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use gix_object :: Kind :: * ; let type_name = match self . kind { Blob => "Blob" , Commit => "Commit" , Tree => "Tree" , Tag => "Tag" , } ; write ! (f , "{}({})" , type_name , self . id) } }
};
}
