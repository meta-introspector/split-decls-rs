// Generated macro for impl_358 (impl)
macro_rules! Depcrate_registryimpl_358 {
() => {
// Module: crate::registry
// Provides: {"impl_358"}
// Dependencies: {}
impl Debug for Registry < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . debug_struct ("Handlebars") . field ("templates" , & self . templates) . field ("helpers" , & self . helpers . keys ()) . field ("decorators" , & self . decorators . keys ()) . field ("strict_mode" , & self . strict_mode) . field ("dev_mode" , & self . dev_mode) . finish () } }
};
}
