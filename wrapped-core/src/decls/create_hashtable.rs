macro_rules! deps {
    () => {
        HashTable!();
    };
}

macro_rules! create_hashtable {
    () => {
        deps!();
        # [doc = " Returns a reference to the latest hash table, creating one if it doesn't exist yet."] # [doc = " The reference is valid forever. However, the `HashTable` it references might become stale"] # [doc = " at any point. Meaning it still exists, but it is not the instance in active use."] # [cold] fn create_hashtable () -> & 'static HashTable { let new_table = Box :: into_raw (HashTable :: new (LOAD_FACTOR , ptr :: null ())) ; let table = match HASHTABLE . compare_exchange (ptr :: null_mut () , new_table , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => new_table , Err (old_table) => { unsafe { let _ = Box :: from_raw (new_table) ; } old_table } } ; unsafe { & * table } }
    };
}

create_hashtable!();