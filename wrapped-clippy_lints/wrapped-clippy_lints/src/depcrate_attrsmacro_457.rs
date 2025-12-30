// Generated macro for macro_457 (macro)
macro_rules! Depcrate_attrsmacro_457 {
() => {
// Module: crate::attrs
// Provides: {"macro_457"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for items with `#[repr(packed)]`-attribute without ABI qualification"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Without qualification, `repr(packed)` implies `repr(Rust)`. The Rust-ABI is inherently unstable."] # [doc = " While this is fine as long as the type is accessed correctly within Rust-code, most uses"] # [doc = " of `#[repr(packed)]` involve FFI and/or data structures specified by network-protocols or"] # [doc = " other external specifications. In such situations, the unstable Rust-ABI implied in"] # [doc = " `#[repr(packed)]` may lead to future bugs should the Rust-ABI change."] # [doc = ""] # [doc = " In case you are relying on a well defined and stable memory layout, qualify the type's"] # [doc = " representation using the `C`-ABI. Otherwise, if the type in question is only ever"] # [doc = " accessed from Rust-code according to Rust's rules, use the `Rust`-ABI explicitly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[repr(packed)]"] # [doc = " struct NetworkPacketHeader {"] # [doc = "     header_length: u8,"] # [doc = "     header_version: u16"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[repr(C, packed)]"] # [doc = " struct NetworkPacketHeader {"] # [doc = "     header_length: u8,"] # [doc = "     header_version: u16"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.85.0"] pub REPR_PACKED_WITHOUT_ABI , suspicious , "ensures that `repr(packed)` always comes with a qualified ABI" }
};
}
