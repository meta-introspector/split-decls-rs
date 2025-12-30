// Generated macro for Result (type)
macro_rules! DepcrateResult {
() => {
// Module: crate
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized [`Result`] type for [`OctetsMut`] operations."] # [doc = ""] # [doc = " [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html"] # [doc = " [`OctetsMut`]: struct.OctetsMut.html"] pub type Result < T > = std :: result :: Result < T , BufferTooShortError > ;
};
}
