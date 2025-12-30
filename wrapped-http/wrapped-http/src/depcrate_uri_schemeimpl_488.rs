// Generated macro for impl_488 (impl)
macro_rules! Depcrate_uri_schemeimpl_488 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_488"}
// Dependencies: {}
impl PartialEq for Scheme { fn eq (& self , other : & Scheme) -> bool { use self :: Protocol :: * ; use self :: Scheme2 :: * ; match (& self . inner , & other . inner) { (& Standard (Http) , & Standard (Http)) => true , (& Standard (Https) , & Standard (Https)) => true , (Other (a) , Other (b)) => a . eq_ignore_ascii_case (b) , (& None , _) | (_ , & None) => unreachable ! () , _ => false , } } }
};
}
