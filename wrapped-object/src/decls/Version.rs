macro_rules! deps {
    () => {
        VersionId!();
        VersionData!();
    };
}

macro_rules! Version {
    () => {
        deps!();
        # [doc = " A version for a symbol."] # [derive (Debug)] pub struct Version < 'data > { id : VersionId , # [doc = " The data for this version."] pub data : VersionData < 'data > , # [doc = " Ignore this version when writing the ELF file."] pub delete : bool , }
    };
}

Version!();