// Generated macro for Loader (struct)
macro_rules! Depcrate_loaderLoader {
() => {
// Module: crate::loader
// Provides: {"Loader"}
// Dependencies: {}
# [doc = " A loader for the DWARF data required for a `Context`."] # [doc = ""] # [doc = " For performance reasons, a [`Context`] normally borrows the input data."] # [doc = " However, that means the input data must outlive the `Context`, which"] # [doc = " is inconvenient for long-lived `Context`s."] # [doc = " This loader uses an arena to store the input data, together with the"] # [doc = " `Context` itself. This ensures that the input data lives as long as"] # [doc = " the `Context`."] # [doc = ""] # [doc = " The loader performs some additional tasks:"] # [doc = " - Loads the symbol table from the executable file (see [`Self::find_symbol`])."] # [doc = " - Loads Mach-O dSYM files that are located next to the executable file."] # [doc = " - Locates and loads split DWARF files (DWO and DWP)."] pub struct Loader { internal : LoaderInternal < 'static > , arena_data : Arena < Vec < u8 > > , arena_mmap : Arena < Mmap > , }
};
}
