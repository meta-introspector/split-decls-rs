macro_rules! IsWhitespaceFn {
    () => {
        # [doc = " Function mapping string element to bool denoting whether it's a whitespace defined by user."] pub type IsWhitespaceFn = Box < dyn Fn (String) -> bool > ;
    };
}

IsWhitespaceFn!()