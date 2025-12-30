// Generated macro for impl_55 (impl)
macro_rules! Depcrate_smart_displayimpl_55 {
() => {
// Module: crate::smart_display
// Provides: {"impl_55"}
// Dependencies: {}
impl < T > Clone for Metadata < '_ , T > where T : SmartDisplay , T :: Metadata : Clone , { fn clone (& self) -> Self { Self { unpadded_width : self . unpadded_width , metadata : self . metadata . clone () , _value : self . _value , } } }
};
}
