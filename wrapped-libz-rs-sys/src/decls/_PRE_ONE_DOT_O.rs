macro_rules! _PRE_ONE_DOT_O {
    () => {
        const _PRE_ONE_DOT_O : () = assert ! ("0" . as_bytes () [0] == b'0') ;
    };
}

_PRE_ONE_DOT_O!();