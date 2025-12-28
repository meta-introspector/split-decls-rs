macro_rules! deps {
    () => {
        RenameError!();
    };
}

macro_rules! _format_err {
    () => {
        deps!();
        # [macro_export] macro_rules ! _format_err { ($ fmt : expr) => { RenameError (format ! ($ fmt)) } ; ($ fmt : expr , $ ($ arg : tt) +) => { RenameError (format ! ($ fmt , $ ($ arg) +)) } }
    };
}

_format_err!();