macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMPORT_OBJECT_ORDINAL {
    () => {
        deps!();
        # [doc = " Import by ordinal"] pub const IMPORT_OBJECT_ORDINAL : u16 = 0 ;
    };
}

IMPORT_OBJECT_ORDINAL!()