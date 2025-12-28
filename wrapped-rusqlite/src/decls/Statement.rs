macro_rules! deps {
    () => {
        Connection!();
        RawStatement!();
    };
}

macro_rules! Statement {
    () => {
        deps!();
        # [doc = " A prepared statement."] pub struct Statement < 'conn > { pub (crate) conn : & 'conn Connection , pub (crate) stmt : RawStatement , }
    };
}

Statement!()