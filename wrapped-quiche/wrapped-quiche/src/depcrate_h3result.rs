// Generated macro for Result (type)
macro_rules! Depcrate_h3Result {
() => {
// Module: crate::h3
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A specialized [`Result`] type for quiche HTTP/3 operations."] # [doc = ""] # [doc = " This type is used throughout quiche's HTTP/3 public API for any operation"] # [doc = " that can produce an error."] # [doc = ""] # [doc = " [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html"] pub type Result < T > = std :: result :: Result < T , Error > ;
};
}
