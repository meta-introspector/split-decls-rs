// Generated macro for seals (module)
macro_rules! Depcrate_traitsseals {
() => {
// Module: crate::traits
// Provides: {"seals"}
// Dependencies: {}
# [doc = " Private module for supertraits of sealed traits."] mod seals { pub trait EncodableLayout { } impl EncodableLayout for [u8] { } impl EncodableLayout for [u16] { } impl EncodableLayout for [f32] { } }
};
}
