// Generated macro for private (module)
macro_rules! Depcrate_streamprivate {
() => {
// Module: crate::stream
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { use std :: io :: Write ; pub trait Sealed { } impl < W : Write > Sealed for super :: BinaryWriter < W > { } impl < W : Write > Sealed for super :: XmlWriter < W > { } }
};
}
