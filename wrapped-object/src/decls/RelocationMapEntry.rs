macro_rules! RelocationMapEntry {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] struct RelocationMapEntry { implicit_addend : bool , addend : u64 , }
    };
}

RelocationMapEntry!();