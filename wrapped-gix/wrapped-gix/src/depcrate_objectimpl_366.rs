// Generated macro for impl_366 (impl)
macro_rules! Depcrate_objectimpl_366 {
() => {
// Module: crate::object
// Provides: {"impl_366"}
// Dependencies: {}
impl std :: fmt :: Debug for ObjectDetached { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use gix_object :: Kind :: * ; let type_name = match self . kind { Blob => "Blob" , Commit => "Commit" , Tree => "Tree" , Tag => "Tag" , } ; write ! (f , "{}({})" , type_name , self . id) } }
};
}
