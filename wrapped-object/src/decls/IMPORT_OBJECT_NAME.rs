macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! IMPORT_OBJECT_NAME {
    () => {
        deps!();
        # [doc = " Import name == public symbol name."] pub const IMPORT_OBJECT_NAME : u16 = 1 ;
    };
}

IMPORT_OBJECT_NAME!()