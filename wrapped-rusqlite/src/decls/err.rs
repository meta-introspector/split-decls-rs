macro_rules! err {
    () => {
        macro_rules ! err { ($ code : expr $ (,) ?) => { $ crate :: error :: error_from_sqlite_code ($ code , None) } ; ($ code : expr , $ msg : literal $ (,) ?) => { $ crate :: error :: error_from_sqlite_code ($ code , Some (format ! ($ msg))) } ; ($ code : expr , $ err : expr $ (,) ?) => { $ crate :: error :: error_from_sqlite_code ($ code , Some (format ! ($ err))) } ; ($ code : expr , $ fmt : expr , $ ($ arg : tt) *) => { $ crate :: error :: error_from_sqlite_code ($ code , Some (format ! ($ fmt , $ ($ arg) *))) } ; }
    };
}

err!();