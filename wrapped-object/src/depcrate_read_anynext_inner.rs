// Generated macro for next_inner (macro)
macro_rules! Depcrate_read_anynext_inner {
() => {
// Module: crate::read::any
// Provides: {"next_inner"}
// Dependencies: {}
# [doc = " Call `next` for a file format iterator."] macro_rules ! next_inner { ($ inner : expr , $ from : ident , $ to : ident) => { match $ inner { # [cfg (feature = "coff")] $ from :: Coff (ref mut iter) => iter . next () . map ($ to :: Coff) , # [cfg (feature = "coff")] $ from :: CoffBig (ref mut iter) => iter . next () . map ($ to :: CoffBig) , # [cfg (feature = "elf")] $ from :: Elf32 (ref mut iter) => iter . next () . map ($ to :: Elf32) , # [cfg (feature = "elf")] $ from :: Elf64 (ref mut iter) => iter . next () . map ($ to :: Elf64) , # [cfg (feature = "macho")] $ from :: MachO32 (ref mut iter) => iter . next () . map ($ to :: MachO32) , # [cfg (feature = "macho")] $ from :: MachO64 (ref mut iter) => iter . next () . map ($ to :: MachO64) , # [cfg (feature = "pe")] $ from :: Pe32 (ref mut iter) => iter . next () . map ($ to :: Pe32) , # [cfg (feature = "pe")] $ from :: Pe64 (ref mut iter) => iter . next () . map ($ to :: Pe64) , # [cfg (feature = "wasm")] $ from :: Wasm (ref mut iter) => iter . next () . map ($ to :: Wasm) , # [cfg (feature = "xcoff")] $ from :: Xcoff32 (ref mut iter) => iter . next () . map ($ to :: Xcoff32) , # [cfg (feature = "xcoff")] $ from :: Xcoff64 (ref mut iter) => iter . next () . map ($ to :: Xcoff64) , } } ; }
};
}
