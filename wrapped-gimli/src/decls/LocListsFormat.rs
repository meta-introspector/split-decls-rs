macro_rules! LocListsFormat {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum LocListsFormat { # [doc = " The bare location list format used before DWARF 5."] Bare , # [doc = " The DW_LLE encoded range list format used in DWARF 5 and the non-standard GNU"] # [doc = " split dwarf extension."] Lle , }
    };
}

LocListsFormat!();