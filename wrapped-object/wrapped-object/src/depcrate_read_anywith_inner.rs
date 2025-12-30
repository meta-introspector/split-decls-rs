// Generated macro for with_inner (macro)
macro_rules! Depcrate_read_anywith_inner {
() => {
// Module: crate::read::any
// Provides: {"with_inner"}
// Dependencies: {}
# [doc = " Evaluate an expression on the contents of a file format enum."] # [doc = ""] # [doc = " This is a hack to avoid virtual calls."] macro_rules ! with_inner { ($ inner : expr , $ enum : ident , | $ var : ident | $ body : expr) => { match $ inner { # [cfg (feature = "coff")] $ enum :: Coff (ref $ var) => $ body , # [cfg (feature = "coff")] $ enum :: CoffBig (ref $ var) => $ body , # [cfg (feature = "elf")] $ enum :: Elf32 (ref $ var) => $ body , # [cfg (feature = "elf")] $ enum :: Elf64 (ref $ var) => $ body , # [cfg (feature = "macho")] $ enum :: MachO32 (ref $ var) => $ body , # [cfg (feature = "macho")] $ enum :: MachO64 (ref $ var) => $ body , # [cfg (feature = "pe")] $ enum :: Pe32 (ref $ var) => $ body , # [cfg (feature = "pe")] $ enum :: Pe64 (ref $ var) => $ body , # [cfg (feature = "wasm")] $ enum :: Wasm (ref $ var) => $ body , # [cfg (feature = "xcoff")] $ enum :: Xcoff32 (ref $ var) => $ body , # [cfg (feature = "xcoff")] $ enum :: Xcoff64 (ref $ var) => $ body , } } ; }
};
}
