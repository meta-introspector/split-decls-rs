// Generated macro for sealed (module)
macro_rules! Depcrate_common_content_encodingsealed {
() => {
// Module: crate::common::content_encoding
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { pub trait AsCoding : Sealed { } pub trait Sealed { fn as_coding (& self) -> & str ; } impl AsCoding for & str { } impl Sealed for & str { fn as_coding (& self) -> & str { self } } }
};
}
