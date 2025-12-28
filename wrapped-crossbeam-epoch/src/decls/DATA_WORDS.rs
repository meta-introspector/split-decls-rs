macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! DATA_WORDS {
    () => {
        deps!();
        # [doc = " Number of words a piece of `Data` can hold."] # [doc = ""] # [doc = " Three words should be enough for the majority of cases. For example, you can fit inside it the"] # [doc = " function pointer together with a fat pointer representing an object that needs to be destroyed."] const DATA_WORDS : usize = 3 ;
    };
}

DATA_WORDS!();