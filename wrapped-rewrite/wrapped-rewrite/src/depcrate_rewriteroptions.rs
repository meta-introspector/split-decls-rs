// Generated macro for Options (struct)
macro_rules! Depcrate_rewriterOptions {
() => {
// Module: crate::rewriter
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for modifying a file."] # [doc = ""] # [doc = " This is used as an argument to the [`Rewriter::modify`] method."] # [doc = ""] # [doc = " The options are listed in the order they are processed."] # [derive (Debug , Default)] # [non_exhaustive] pub struct Options { # [doc = " Delete symbols from the symbol table."] # [doc = ""] # [doc = " See [`Rewriter::delete_symbols`]."] pub delete_symbols : HashSet < Vec < u8 > > , # [doc = " Rename symbols in the symbol table."] # [doc = ""] # [doc = " See [`Rewriter::rename_symbols`]."] pub rename_symbols : HashMap < Vec < u8 > , Vec < u8 > > , # [doc = " Delete sections from the file."] # [doc = ""] # [doc = " See [`Rewriter::delete_sections`]."] pub delete_sections : HashSet < Vec < u8 > > , # [doc = " Rename sections in the file."] # [doc = ""] # [doc = " See [`Rewriter::rename_sections`]."] pub rename_sections : HashMap < Vec < u8 > , Vec < u8 > > , # [doc = " Options that are specific to ELF files."] pub elf : super :: ElfOptions , }
};
}
