macro_rules! deps {
    () => {
        SymbolSection!();
        Section!();
        SectionId!();
    };
}

macro_rules! impl_1053 {
    () => {
        deps!();
        impl SymbolSection { # [doc = " Returns the section id for the section where the symbol is defined."] # [doc = ""] # [doc = " May return `None` if the symbol is not defined in a section."] # [inline] pub fn id (self) -> Option < SectionId > { if let SymbolSection :: Section (id) = self { Some (id) } else { None } } }
    };
}

impl_1053!()