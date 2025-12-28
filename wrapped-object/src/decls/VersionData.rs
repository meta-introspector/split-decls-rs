macro_rules! deps {
    () => {
        VersionNeed!();
        VersionDef!();
    };
}

macro_rules! VersionData {
    () => {
        deps!();
        # [doc = " The data for a version for a symbol."] # [derive (Debug)] pub enum VersionData < 'data > { # [doc = " The version for a defined symbol."] Def (VersionDef < 'data >) , # [doc = " The version for an undefined symbol."] Need (VersionNeed < 'data >) , }
    };
}

VersionData!();