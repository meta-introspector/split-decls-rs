macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! color_err {
    () => {
        deps!();
        fn color_err (input : impl Into < BString >) -> Error { Error :: new ("Colors are specific color values and their attributes, like 'brightred', or 'blue'" , input ,) }
    };
}

color_err!()