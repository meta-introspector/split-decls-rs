macro_rules! deps {
    () => {
        VersionDef!();
        VersionNeed!();
    };
}

macro_rules! VersionData {
    () => {
        deps!();
        # [doc = " The data for a version for a symbol."] # [derive (Debug)] pub enum VersionData < 'data > { # [doc = " The version for a defined symbol."] Def (VersionDef < 'data >) , # [doc = " The version for an undefined symbol."] Need (VersionNeed < 'data >) , }
    };
}

VersionData!()