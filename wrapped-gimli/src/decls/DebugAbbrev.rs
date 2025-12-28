macro_rules! DebugAbbrev {
    () => {
        # [doc = " The `DebugAbbrev` struct represents the abbreviations describing"] # [doc = " `DebuggingInformationEntry`s' attribute names and forms found in the"] # [doc = " `.debug_abbrev` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugAbbrev < R > { debug_abbrev_section : R , }
    };
}

DebugAbbrev!();