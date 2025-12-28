macro_rules! deps {
    () => {
        Token!();
        TokenSource!();
    };
}

macro_rules! Interner {
    () => {
        deps!();
        # [doc = " An interner that allows for fast access of tokens produced by a [`TokenSource`]."] # [derive (Default)] pub struct Interner < T > { tokens : Vec < T > , table : HashTable < Token > , hasher : RandomState , }
    };
}

Interner!()