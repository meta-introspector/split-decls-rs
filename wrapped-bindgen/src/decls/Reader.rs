macro_rules! deps {
    () => {
        File!();
        Type!();
    };
}

macro_rules! Reader {
    () => {
        deps!();
        pub struct Reader (HashMap < & 'static str , HashMap < & 'static str , Vec < Type > > > , Vec < * mut File > ,) ;
    };
}

Reader!()