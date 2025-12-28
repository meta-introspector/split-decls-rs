macro_rules! RelocationSections {
    () => {
        # [doc = " A mapping from section index to associated relocation sections."] # [derive (Debug , Default)] pub struct RelocationSections { relocations : Vec < usize > , }
    };
}

RelocationSections!();