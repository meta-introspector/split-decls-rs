macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMPORT_OBJECT_NAME_UNDECORATE {
    () => {
        deps!();
        # [doc = " Import name == public symbol name skipping leading ?, @, or optionally _ and truncating at first @."] pub const IMPORT_OBJECT_NAME_UNDECORATE : u16 = 3 ;
    };
}

IMPORT_OBJECT_NAME_UNDECORATE!()