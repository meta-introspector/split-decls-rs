macro_rules! deps {
    () => {
        Statement!();
    };
}

macro_rules! Row {
    () => {
        deps!();
        # [doc = " A single result row of a query."] pub struct Row < 'stmt > { pub (crate) stmt : & 'stmt Statement < 'stmt > , }
    };
}

Row!();