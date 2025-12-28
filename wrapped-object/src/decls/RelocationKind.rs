macro_rules! deps {
    () => {
        Note!();
        SectionIndex!();
        Section!();
    };
}

macro_rules! RelocationKind {
    () => {
        deps!();
        # [doc = " The operation used to calculate the result of the relocation."] # [doc = ""] # [doc = " The relocation descriptions use the following definitions. Note that"] # [doc = " these definitions probably don't match any ELF ABI."] # [doc = ""] # [doc = " * A - The value of the addend."] # [doc = " * G - The address of the symbol's entry within the global offset table."] # [doc = " * L - The address of the symbol's entry within the procedure linkage table."] # [doc = " * P - The address of the place of the relocation."] # [doc = " * S - The address of the symbol."] # [doc = " * GotBase - The address of the global offset table."] # [doc = " * Image - The base address of the image."] # [doc = " * Section - The address of the section containing the symbol."] # [doc = ""] # [doc = " 'XxxRelative' means 'Xxx + A - P'.  'XxxOffset' means 'S + A - Xxx'."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum RelocationKind { # [doc = " The operation is unknown."] Unknown , # [doc = " S + A"] Absolute , # [doc = " S + A - P"] Relative , # [doc = " G + A - GotBase"] Got , # [doc = " G + A - P"] GotRelative , # [doc = " GotBase + A - P"] GotBaseRelative , # [doc = " S + A - GotBase"] GotBaseOffset , # [doc = " L + A - P"] PltRelative , # [doc = " S + A - Image"] ImageOffset , # [doc = " S + A - Section"] SectionOffset , # [doc = " The index of the section containing the symbol."] SectionIndex , }
    };
}

RelocationKind!()