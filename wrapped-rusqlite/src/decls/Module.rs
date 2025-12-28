macro_rules! deps {
    () => {
        VTab!();
    };
}

macro_rules! Module {
    () => {
        deps!();
        # [doc = " Virtual table module"] # [doc = ""] # [doc = " (See [SQLite doc](https://sqlite.org/c3ref/module.html))"] # [repr (transparent)] pub struct Module < 'vtab , T : VTab < 'vtab > > { base : ffi :: sqlite3_module , phantom : PhantomData < & 'vtab T > , }
    };
}

Module!()