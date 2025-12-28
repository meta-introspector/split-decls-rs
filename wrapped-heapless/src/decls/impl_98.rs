macro_rules! deps {
    () => {
        VacantEntry!();
        Insert!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'a , K , V , const N : usize > VacantEntry < 'a , K , V , N > where K : Eq + Hash , { # [doc = " Get the key associated with this entry"] pub fn key (& self) -> & K { & self . key } # [doc = " Consumes this entry to yield to key associated with it"] pub fn into_key (self) -> K { self . key } # [doc = " Inserts this entry into to underlying map, yields a mutable reference to the inserted value."] # [doc = " If the map is at capacity the value is returned instead."] pub fn insert (self , value : V) -> Result < & 'a mut V , V > { if self . core . entries . is_full () { Err (value) } else { match self . core . insert (self . hash_val , self . key , value) { Insert :: Success (inserted) => { unsafe { Ok (& mut (* self . core . entries . as_mut_ptr () . add (inserted . index)) . value) } } Insert :: Full ((_ , v)) => Err (v) , } } } }
    };
}

impl_98!();