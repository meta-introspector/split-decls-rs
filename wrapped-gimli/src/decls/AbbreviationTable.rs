macro_rules! deps {
    () => {
        Abbreviation!();
        FnvIndexSet!();
    };
}

macro_rules! AbbreviationTable {
    () => {
        deps!();
        # [doc = " A table of abbreviations that will be stored in a `.debug_abbrev` section."] # [derive (Debug , Default)] pub (crate) struct AbbreviationTable { abbrevs : FnvIndexSet < Abbreviation > , }
    };
}

AbbreviationTable!();