macro_rules! VTabConnection {
    () => {
        # [doc = " `feature = \"vtab\"`"] pub struct VTabConnection (* mut ffi :: sqlite3) ;
    };
}

VTabConnection!()