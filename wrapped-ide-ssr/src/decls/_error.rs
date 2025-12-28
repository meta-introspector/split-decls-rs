macro_rules! deps {
    () => {
        SsrError!();
    };
}

macro_rules! _error {
    () => {
        deps!();
        # [doc = " Constructs an SsrError taking arguments like the format macro."] macro_rules ! _error { ($ fmt : expr) => { $ crate :: SsrError :: new (format ! ($ fmt)) } ; ($ fmt : expr , $ ($ arg : tt) +) => { $ crate :: SsrError :: new (format ! ($ fmt , $ ($ arg) +)) } }
    };
}

_error!()