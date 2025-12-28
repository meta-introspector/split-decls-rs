macro_rules! deps {
    () => {
        Type!();
        File!();
    };
}

macro_rules! Reader {
    () => {
        deps!();
        pub struct Reader (HashMap < & 'static str , HashMap < & 'static str , Vec < Type > > > , Vec < * mut File > ,) ;
    };
}

Reader!();