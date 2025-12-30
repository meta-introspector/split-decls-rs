// Generated macro for ForeignMod (struct)
macro_rules! Depcrate_astForeignMod {
() => {
// Module: crate::ast
// Provides: {"ForeignMod"}
// Dependencies: {}
# [doc = " Foreign module declaration."] # [doc = ""] # [doc = " E.g., `extern { .. }` or `extern \"C\" { .. }`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct ForeignMod { # [doc = " Span of the `extern` keyword."] pub extern_span : Span , # [doc = " `unsafe` keyword accepted syntactically for macro DSLs, but not"] # [doc = " semantically by Rust."] pub safety : Safety , pub abi : Option < StrLit > , pub items : ThinVec < Box < ForeignItem > > , }
};
}
