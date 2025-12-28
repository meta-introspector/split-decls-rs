macro_rules! IndexInfo {
    () => {
        # [doc = " Pass information into and receive the reply from the"] # [doc = " [`VTab::best_index`] method."] # [doc = ""] # [doc = " (See [SQLite doc](http://sqlite.org/c3ref/index_info.html))"] # [derive (Debug)] pub struct IndexInfo (* mut ffi :: sqlite3_index_info) ;
    };
}

IndexInfo!();