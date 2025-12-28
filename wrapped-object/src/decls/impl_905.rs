macro_rules! deps {
    () => {
        SymbolSection!();
        Section!();
        SectionIndex!();
    };
}

macro_rules! impl_905 {
    () => {
        deps!();
        impl SymbolSection { # [doc = " Returns the section index for the section where the symbol is defined."] # [doc = ""] # [doc = " May return `None` if the symbol is not defined in a section."] # [inline] pub fn index (self) -> Option < SectionIndex > { if let SymbolSection :: Section (index) = self { Some (index) } else { None } } }
    };
}

impl_905!();