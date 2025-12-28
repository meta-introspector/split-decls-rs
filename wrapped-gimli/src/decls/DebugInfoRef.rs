macro_rules! DebugInfoRef {
    () => {
        # [doc = " A reference to a `.debug_info` entry."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum DebugInfoRef { # [doc = " An external symbol."] # [doc = ""] # [doc = " The meaning of this value is decided by the writer, but"] # [doc = " will typically be an index into a symbol table."] Symbol (usize) , # [doc = " An entry in the same section."] # [doc = ""] # [doc = " This only supports references in units that are emitted together."] Entry (UnitId , UnitEntryId) , }
    };
}

DebugInfoRef!();