macro_rules! deps {
    () => {
        StoreBulkMut!();
        StoreIntoIterator!();
        StoreIterableMut!();
        StoreConstEmpty!();
        StoreFromIterator!();
    };
}

macro_rules! test_extend {
    () => {
        deps!();
        fn test_extend < 'a , S > () where S : StoreConstEmpty < u32 , u64 > + StoreIterableMut < 'a , u32 , u64 > + StoreIntoIterator < u32 , u64 > + StoreFromIterator < u32 , u64 > + StoreBulkMut < u32 , u64 > + Clone + Debug + PartialEq + 'a , { let mut map : LiteMap < u32 , u64 , S > = LiteMap :: new () ; let initial_entries = [(1 , 1) , (2 , 2) , (3 , 3)] ; map . extend (initial_entries) ; assert_eq ! (map . len () , 3) ; assert_eq ! (map . get (& 1) , Some (& 1)) ; assert_eq ! (map . get (& 2) , Some (& 2)) ; assert_eq ! (map . get (& 3) , Some (& 3)) ; let overlapping_entries = [(2 , 22) , (4 , 44) , (1 , 11)] ; map . extend (overlapping_entries) ; assert_eq ! (map . len () , 4) ; assert_eq ! (map . get (& 1) , Some (& 11)) ; assert_eq ! (map . get (& 2) , Some (& 22)) ; assert_eq ! (map . get (& 3) , Some (& 3)) ; assert_eq ! (map . get (& 4) , Some (& 44)) ; let duplicate_entries = [(3 , 333) , (3 , 3333) , (5 , 5)] ; map . extend (duplicate_entries) ; assert_eq ! (map . len () , 5) ; assert_eq ! (map . get (& 3) , Some (& 3333)) ; assert_eq ! (map . get (& 5) , Some (& 5)) ; let empty_entries : Vec < (u32 , u64) > = Vec :: new () ; let map_clone = map . clone () ; map . extend (empty_entries) ; check_equivalence (map . values . clone () , map_clone . values . clone ()) ; map . extend (map_clone . clone ()) ; check_equivalence (map . values . clone () , map_clone . values) ; }
    };
}

test_extend!();