// Generated macro for impl_54 (impl)
macro_rules! Depcrate_smart_displayimpl_54 {
() => {
// Module: crate::smart_display
// Provides: {"impl_54"}
// Dependencies: {}
impl < T > Debug for Metadata < '_ , T > where T : SmartDisplay , T :: Metadata : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { f . debug_struct ("Metadata") . field ("unpadded_width" , & self . unpadded_width) . field ("metadata" , & self . metadata) . finish () } }
};
}
