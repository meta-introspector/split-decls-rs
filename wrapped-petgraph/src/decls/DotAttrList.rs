macro_rules! DotAttrList {
    () => {
        pub type DotAttrList < 'a > = AList < (& 'a str , & 'a str) > ;
    };
}

DotAttrList!();