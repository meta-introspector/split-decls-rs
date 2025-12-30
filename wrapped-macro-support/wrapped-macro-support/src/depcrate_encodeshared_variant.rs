// Generated macro for shared_variant (function)
macro_rules! Depcrate_encodeshared_variant {
() => {
// Module: crate::encode
// Provides: {"shared_variant"}
// Dependencies: {}
fn shared_variant < 'a > (v : & 'a ast :: Variant , intern : & 'a Interner) -> EnumVariant < 'a > { EnumVariant { name : intern . intern (& v . name) , value : v . value , comments : v . comments . iter () . map (| s | & * * s) . collect () , } }
};
}
