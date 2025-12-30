// Generated macro for impl_744 (impl)
macro_rules! Depcrate_objectimpl_744 {
() => {
// Module: crate::object
// Provides: {"impl_744"}
// Dependencies: {}
impl < 'repo > std :: fmt :: Debug for Object < 'repo > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { let mut ds = f . debug_struct ("Object") ; match self . kind () { Some (kind) => ds . field ("kind" , & kind) , None => ds . field ("kind" , & format ! ("Unknow ({})" , unsafe { raw :: git_object_type (&* self . raw) }) ,) , } ; ds . field ("id" , & self . id ()) ; ds . finish () } }
};
}
