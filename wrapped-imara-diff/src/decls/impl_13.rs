macro_rules! deps {
    () => {
        Token!();
        InternedInput!();
        TokenSource!();
        Interner!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T : Eq + Hash > InternedInput < T > { pub fn new < I : TokenSource < Token = T > > (before : I , after : I) -> Self { let token_estimate_before = before . estimate_tokens () as usize ; let token_estimate_after = after . estimate_tokens () as usize ; let mut res = Self { before : Vec :: with_capacity (token_estimate_before) , after : Vec :: with_capacity (token_estimate_after) , interner : Interner :: new (token_estimate_before + token_estimate_after) , } ; res . update_before (before . tokenize ()) ; res . update_after (after . tokenize ()) ; res } # [doc = " Create an Interner with an intial capacity calculated by calling"] # [doc = " [`estimate_tokens`](crate::intern::TokenSource::estimate_tokens) methods of `before` and `after`"] pub fn reserve_for_token_source < S : TokenSource < Token = T > + ? Sized > (& mut self , before : & S , after : & S ,) { self . reserve (before . estimate_tokens () , after . estimate_tokens ()) } pub fn reserve (& mut self , capacity_before : u32 , capacity_after : u32) { self . before . reserve (capacity_before as usize) ; self . after . reserve (capacity_after as usize) ; self . interner . reserve (capacity_before as usize + capacity_after as usize) ; } # [doc = " replaces `self.before` with the interned Tokens yielded by `input`"] # [doc = " Note that this does not erase any tokens from the interner and might therefore be considered"] # [doc = " a memory leak. If this function is called often over a long_running process"] # [doc = " consider clearing the interner with [`clear`](crate::intern::Interner::clear)."] pub fn update_before (& mut self , input : impl Iterator < Item = T >) { self . before . clear () ; self . before . extend (input . map (| token | self . interner . intern (token))) ; } # [doc = " replaces `self.before` with the interned Tokens yielded by `input`"] # [doc = " Note that this does not erase any tokens from the interner and might therefore be considered"] # [doc = " a memory leak. If this function is called often over a long_running process"] # [doc = " consider clearing the interner with [`clear`](crate::intern::Interner::clear) or"] # [doc = " [`erase_tokens_after`](crate::intern::Interner::erase_tokens_after)."] pub fn update_after (& mut self , input : impl Iterator < Item = T >) { self . after . clear () ; self . after . extend (input . map (| token | self . interner . intern (token))) ; } }
    };
}

impl_13!();