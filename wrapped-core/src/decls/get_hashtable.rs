macro_rules! deps {
    () => {
        HashTable!();
    };
}

macro_rules! get_hashtable {
    () => {
        deps!();
        # [doc = " Returns a reference to the latest hash table, creating one if it doesn't exist yet."] # [doc = " The reference is valid forever. However, the `HashTable` it references might become stale"] # [doc = " at any point. Meaning it still exists, but it is not the instance in active use."] # [inline] fn get_hashtable () -> & 'static HashTable { let table = HASHTABLE . load (Ordering :: Acquire) ; if table . is_null () { create_hashtable () } else { unsafe { & * table } } }
    };
}

get_hashtable!()