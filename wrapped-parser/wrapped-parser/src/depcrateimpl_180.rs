// Generated macro for impl_180 (impl)
macro_rules! Depcrateimpl_180 {
() => {
// Module: crate
// Provides: {"impl_180"}
// Dependencies: {}
impl Serialize for ErrorPositions { fn serialize < S : Serializer > (& self , serializer : S) -> std :: result :: Result < S :: Ok , S :: Error > { serializer . collect_seq (self . clone ()) } }
};
}
