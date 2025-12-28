macro_rules! AutoImportExclusionType {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum AutoImportExclusionType { Always , Methods , }
    };
}

AutoImportExclusionType!();