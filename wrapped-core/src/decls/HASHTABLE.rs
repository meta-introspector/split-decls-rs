macro_rules! deps {
    () => {
        HashTable!();
    };
}

macro_rules! HASHTABLE {
    () => {
        deps!();
        # [doc = " Holds the pointer to the currently active `HashTable`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Except for the initial value of null, it must always point to a valid `HashTable` instance."] # [doc = " Any `HashTable` this global static has ever pointed to must never be freed."] static HASHTABLE : AtomicPtr < HashTable > = AtomicPtr :: new (ptr :: null_mut ()) ;
    };
}

HASHTABLE!()