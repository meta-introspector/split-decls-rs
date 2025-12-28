macro_rules! RangeListsFormat {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum RangeListsFormat { # [doc = " The bare range list format used before DWARF 5."] Bare , # [doc = " The DW_RLE encoded range list format used in DWARF 5."] Rle , }
    };
}

RangeListsFormat!()