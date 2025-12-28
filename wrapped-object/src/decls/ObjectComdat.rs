macro_rules! deps {
    () => {
        ComdatKind!();
        Item!();
        Object!();
        SymbolIndex!();
        SectionIterator!();
        Result!();
        SectionIndex!();
    };
}

macro_rules! ObjectComdat {
    () => {
        deps!();
        # [doc = " A COMDAT section group in an [`Object`]."] # [doc = ""] # [doc = " This trait is part of the unified read API."] pub trait ObjectComdat < 'data > : read :: private :: Sealed { # [doc = " An iterator for the sections in the section group."] type SectionIterator : Iterator < Item = SectionIndex > ; # [doc = " Returns the COMDAT selection kind."] fn kind (& self) -> ComdatKind ; # [doc = " Returns the index of the symbol used for the name of COMDAT section group."] fn symbol (& self) -> SymbolIndex ; # [doc = " Returns the name of the COMDAT section group."] fn name_bytes (& self) -> Result < & 'data [u8] > ; # [doc = " Returns the name of the COMDAT section group."] # [doc = ""] # [doc = " Returns an error if the name is not UTF-8."] fn name (& self) -> Result < & 'data str > ; # [doc = " Get the sections in this section group."] fn sections (& self) -> Self :: SectionIterator ; }
    };
}

ObjectComdat!();