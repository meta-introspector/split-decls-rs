macro_rules! deps {
    () => {
        Section!();
        SectionId!();
    };
}

macro_rules! RelocationTarget {
    () => {
        deps!();
        # [doc = " The target of a relocation."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum RelocationTarget { # [doc = " The relocation target is a symbol."] # [doc = ""] # [doc = " The meaning of this value is decided by the writer, but"] # [doc = " will typically be an index into a symbol table."] Symbol (usize) , # [doc = " The relocation target is a section."] Section (SectionId) , }
    };
}

RelocationTarget!();