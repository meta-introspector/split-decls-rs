macro_rules! deps {
    () => {
        Interner!();
        Token!();
        TokenSource!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : Hash + Eq > Interner < T > { # [doc = " Create an Interner with an intial capacity calculated by calling"] # [doc = " [`estimate_tokens`](crate::intern::TokenSource::estimate_tokens) methods of `before` and `after`"] pub fn reserve_for_token_source < S : TokenSource < Token = T > > (& mut self , before : & S , after : & S) { self . reserve (before . estimate_tokens () as usize + after . estimate_tokens () as usize) } pub fn reserve (& mut self , capacity : usize) { self . table . reserve (capacity , | & token | { self . hasher . hash_one (& self . tokens [token . 0 as usize]) }) ; self . tokens . reserve (capacity) ; } # [doc = " Intern `token` and return a the interned integer."] pub fn intern (& mut self , token : T) -> Token { let hash = self . hasher . hash_one (& token) ; match self . table . entry (hash , | & it | self . tokens [it . 0 as usize] == token , | & token | self . hasher . hash_one (& self . tokens [token . 0 as usize]) ,) { Entry :: Occupied (entry) => * entry . get () , Entry :: Vacant (entry) => { let interned = Token (self . tokens . len () as u32) ; entry . insert (interned) ; self . tokens . push (token) ; interned } } } # [doc = " Erases `first_erased_token` and any tokens interned afterward from the interner."] pub fn erase_tokens_after (& mut self , first_erased_token : Token) { assert ! (first_erased_token . 0 <= self . tokens . len () as u32) ; let retained = first_erased_token . 0 as usize ; let erased = self . tokens . len () - retained ; if retained <= erased { self . table . clear () ; for (i , token) in self . tokens [0 .. retained] . iter () . enumerate () { let hash = self . hasher . hash_one (token) ; self . table . insert_unique (hash , Token (i as u32) , | & token | { self . hasher . hash_one (& self . tokens [token . 0 as usize]) }) ; } } else { for (i , token) in self . tokens [retained ..] . iter () . enumerate () { let hash = self . hasher . hash_one (token) ; match self . table . find_entry (hash , | token | token . 0 == (retained + i) as u32) { Ok (occupied) => drop (occupied . remove ()) , Err (_absent) => unreachable ! () , } } } self . tokens . truncate (first_erased_token . 0 as usize) ; } }
    };
}

impl_16!()