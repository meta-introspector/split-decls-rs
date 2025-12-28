macro_rules! kw {
    () => {
        mod kw { syn :: custom_keyword ! (complete) ; }
    };
}

kw!();