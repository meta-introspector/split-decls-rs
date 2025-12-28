macro_rules! SeriesTab {
    () => {
        # [doc = " An instance of the Series virtual table"] # [repr (C)] struct SeriesTab { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab , }
    };
}

SeriesTab!();