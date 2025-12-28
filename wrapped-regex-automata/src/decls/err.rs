macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! err {
    () => {
        deps!();
        macro_rules ! err { ($ msg : expr) => { return Err (DeserializeError :: generic ($ msg)) ; } ; }
    };
}

err!()