macro_rules! deps {
    () => {
        Array!();
    };
}

macro_rules! ArrayTab {
    () => {
        deps!();
        # [doc = " An instance of the Array virtual table"] # [repr (C)] struct ArrayTab { # [doc = " Base class. Must be first"] base : ffi :: sqlite3_vtab , }
    };
}

ArrayTab!();