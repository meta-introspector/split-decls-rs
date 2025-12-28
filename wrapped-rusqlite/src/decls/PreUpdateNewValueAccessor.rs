macro_rules! PreUpdateNewValueAccessor {
    () => {
        # [doc = " An accessor to access the new values of the row being inserted/updated"] # [doc = " during the preupdate callback."] # [derive (Debug)] pub struct PreUpdateNewValueAccessor { db : * mut ffi :: sqlite3 , new_row_id : i64 , }
    };
}

PreUpdateNewValueAccessor!();