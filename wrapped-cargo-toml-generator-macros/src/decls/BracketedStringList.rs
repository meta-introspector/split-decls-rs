macro_rules! BracketedStringList {
    () => {
        pub struct BracketedStringList { pub list : Punctuated < LitStr , Token ! [,] > , }
    };
}

BracketedStringList!()