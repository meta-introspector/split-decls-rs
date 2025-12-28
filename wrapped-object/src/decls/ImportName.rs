macro_rules! deps {
    () => {
        Name!();
        Import!();
    };
}

macro_rules! ImportName {
    () => {
        deps!();
        # [doc = " The name or ordinal to import from a DLL."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum ImportName < 'data > { # [doc = " Import by ordinal. Ordinarily this is a 1-based index."] Ordinal (u16) , # [doc = " Import by name."] Name (& 'data [u8]) , }
    };
}

ImportName!();