macro_rules! deps {
    () => {
        ColorTag!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " Stores all the current open tags encountered in the format string."] # [derive (Debug , PartialEq , Default)] pub struct Context < 'a > (Vec < ColorTag < 'a > >) ;
    };
}

Context!();