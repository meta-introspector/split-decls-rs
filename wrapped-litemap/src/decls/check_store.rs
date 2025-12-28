macro_rules! deps {
    () => {
        StoreFromIterator!();
        StoreMut!();
        StoreIterable!();
        StoreConstEmpty!();
    };
}

macro_rules! check_store {
    () => {
        deps!();
        # [doc = " Tests that a litemap that uses the given store as backend has behavior consistent with the"] # [doc = " reference impl."] # [doc = ""] # [doc = " Call this function in a test with the store impl to test as a valid backend for LiteMap."] # [expect (clippy :: expect_used)] pub fn check_store < 'a , S > () where S : StoreConstEmpty < u32 , u64 > + StoreMut < u32 , u64 > + StoreIterable < 'a , u32 , u64 > + StoreFromIterator < u32 , u64 > + Clone + Debug + PartialEq + 'a , { let mut litemap_test : LiteMap < u32 , u64 , S > = LiteMap :: new () ; assert ! (litemap_test . is_empty ()) ; let mut litemap_std = LiteMap :: < u32 , u64 > :: new () ; populate_litemap (& mut litemap_test) ; populate_litemap (& mut litemap_std) ; check_equivalence (litemap_test . clone () . values , litemap_std . clone () . values) ; litemap_test . remove (& 175) . ok_or (()) . expect_err ("does not exist") ; litemap_test . remove (& 147) . ok_or (()) . expect ("exists") ; litemap_std . remove (& 175) . ok_or (()) . expect_err ("does not exist") ; litemap_std . remove (& 147) . ok_or (()) . expect ("exists") ; assert_eq ! (19 , litemap_test . len ()) ; assert_eq ! (19 , litemap_std . len ()) ; check_equivalence (litemap_test . clone () . values , litemap_std . clone () . values) ; litemap_test . clear () ; litemap_std . clear () ; check_equivalence (litemap_test . values , litemap_std . values) ; }
    };
}

check_store!();