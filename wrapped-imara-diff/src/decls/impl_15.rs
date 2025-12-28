macro_rules! deps {
    () => {
        TokenSource!();
        Interner!();
        Token!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T > Interner < T > { # [doc = " Create an Interner with an initial capacity calculated by summing the results of calling"] # [doc = " [`estimate_tokens`](crate::intern::TokenSource::estimate_tokens) methods of `before` and `after`."] pub fn new_for_token_source < S : TokenSource < Token = T > > (before : & S , after : & S) -> Self { Self :: new (before . estimate_tokens () as usize + after . estimate_tokens () as usize) } # [doc = " Create an Interner with initial capacity `capacity`."] pub fn new (capacity : usize) -> Interner < T > { Interner { tokens : Vec :: with_capacity (capacity) , table : HashTable :: with_capacity (capacity) , hasher : RandomState :: default () , } } # [doc = " Remove all interned tokens."] pub fn clear (& mut self) { self . table . clear () ; self . tokens . clear () ; } # [doc = " Returns to total number of **distinct** tokens currently interned."] pub fn num_tokens (& self) -> u32 { self . tokens . len () as u32 } }
    };
}

impl_15!();