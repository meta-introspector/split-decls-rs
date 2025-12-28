macro_rules! deps {
    () => {
        StoreMut!();
    };
}

macro_rules! StoreIntoIterator {
    () => {
        deps!();
        pub trait StoreIntoIterator < K , V > : StoreMut < K , V > { type KeyValueIntoIter : Iterator < Item = (K , V) > ; # [doc = " Returns an iterator that moves every item from this store."] fn lm_into_iter (self) -> Self :: KeyValueIntoIter ; # [doc = " Adds items from another store to the end of this store."] fn lm_extend_end (& mut self , other : Self) where Self : Sized , { for item in other . lm_into_iter () { self . lm_push (item . 0 , item . 1) ; } } # [doc = " Adds items from another store to the beginning of this store."] fn lm_extend_start (& mut self , other : Self) where Self : Sized , { for (i , item) in other . lm_into_iter () . enumerate () { self . lm_insert (i , item . 0 , item . 1) ; } } }
    };
}

StoreIntoIterator!();