macro_rules! deps {
    () => {
        IdStorage!();
        IdIterator!();
    };
}

macro_rules! impl_1014 {
    () => {
        deps!();
        impl < T , S : BuildHasher > IdStorage < T , S > { fn with_capacity_and_hasher (capacity : usize , hasher : S) -> Self { IdStorage { elements : Vec :: with_capacity (capacity) , upper_bound : 0 , removed_ids : IndexSet :: with_hasher (hasher) , } } fn add (& mut self , element : T) -> usize { let id = if let Some (id) = self . removed_ids . pop () { id } else { let id = self . upper_bound ; self . upper_bound += 1 ; ensure_len (& mut self . elements , id + 1) ; id } ; self . elements [id] = Some (element) ; id } fn remove (& mut self , id : usize) -> T { let data = self . elements [id] . take () . unwrap () ; if self . upper_bound - id == 1 { self . upper_bound -= 1 ; } else { self . removed_ids . insert (id) ; } data } fn clear (& mut self) { self . upper_bound = 0 ; self . elements . clear () ; self . removed_ids . clear () ; } # [inline] fn len (& self) -> usize { self . upper_bound - self . removed_ids . len () } fn iter_ids (& self) -> IdIterator < '_ , S > { IdIterator { upper_bound : self . upper_bound , removed_ids : & self . removed_ids , current : None , } } }
    };
}

impl_1014!()