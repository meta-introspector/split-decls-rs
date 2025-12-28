macro_rules! deps {
    () => {
        KeyValue!();
    };
}

macro_rules! InlineTable {
    () => {
        deps!();
        pub struct InlineTable { pub items : Punctuated < KeyValue , Token ! [,] > , }
    };
}

InlineTable!();