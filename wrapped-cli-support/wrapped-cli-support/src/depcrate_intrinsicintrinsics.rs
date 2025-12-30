// Generated macro for intrinsics (macro)
macro_rules! Depcrate_intrinsicintrinsics {
() => {
// Module: crate::intrinsic
// Provides: {"intrinsics"}
// Dependencies: {}
macro_rules ! intrinsics { (pub enum Intrinsic { $ ($ name : ident = $ sym : literal ,) * }) => { # [doc = " All wasm-bindgen intrinsics that could be depended on by a wasm"] # [doc = " module."] # [derive (Debug)] pub enum Intrinsic { $ ($ name ,) * } impl std :: str :: FromStr for Intrinsic { type Err = anyhow :: Error ; # [doc = " Returns the corresponding intrinsic for a symbol name, if one"] # [doc = " matches."] fn from_str (symbol : & str) -> anyhow :: Result < Intrinsic > { Ok (match symbol { $ ($ sym => Intrinsic ::$ name ,) * _ => anyhow :: bail ! ("unknown intrinsic `{symbol}`") , }) } } } ; }
};
}
