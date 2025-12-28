macro_rules! SmallCString {
    () => {
        # [doc = " Similar to `std::ffi::CString`, but avoids heap allocating if the string is"] # [doc = " small enough. Also guarantees it's input is UTF-8 -- used for cases where we"] # [doc = " need to pass a NUL-terminated string to SQLite, and we have a `&str`."] # [derive (Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct SmallCString (SmallVec < u8 , 16 >) ;
    };
}

SmallCString!()