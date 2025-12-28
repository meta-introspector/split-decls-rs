macro_rules! deps {
    () => {
        Section!();
        SectionId!();
    };
}

macro_rules! SymbolSection {
    () => {
        deps!();
        # [doc = " The section where a symbol is defined."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum SymbolSection { # [doc = " The section is not applicable for this symbol (such as file symbols)."] None , # [doc = " The symbol is undefined."] Undefined , # [doc = " The symbol has an absolute value."] Absolute , # [doc = " The symbol is a zero-initialized symbol that will be combined with duplicate definitions."] Common , # [doc = " The symbol is defined in the given section."] Section (SectionId) , }
    };
}

SymbolSection!();