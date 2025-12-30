// Generated macro for Result (type)
macro_rules! DepcrateResult {
() => {
// Module: crate
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized [`Result`] type for quiche operations."] # [doc = ""] # [doc = " This type is used throughout quiche's public API for any operation that"] # [doc = " can produce an error."] # [doc = ""] # [doc = " [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html"] pub type Result < T > = std :: result :: Result < T , Error > ;
};
}
