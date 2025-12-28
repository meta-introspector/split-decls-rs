macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMPORT_OBJECT_NAME_EXPORTAS {
    () => {
        deps!();
        # [doc = " Import name == a name is explicitly provided after the DLL name."] pub const IMPORT_OBJECT_NAME_EXPORTAS : u16 = 4 ;
    };
}

IMPORT_OBJECT_NAME_EXPORTAS!()