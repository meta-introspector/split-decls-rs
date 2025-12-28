macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMPORT_OBJECT_NAME_NO_PREFIX {
    () => {
        deps!();
        # [doc = " Import name == public symbol name skipping leading ?, @, or optionally _."] pub const IMPORT_OBJECT_NAME_NO_PREFIX : u16 = 2 ;
    };
}

IMPORT_OBJECT_NAME_NO_PREFIX!()