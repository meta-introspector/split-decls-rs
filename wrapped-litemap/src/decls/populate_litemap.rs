macro_rules! deps {
    () => {
        StoreMut!();
    };
}

macro_rules! populate_litemap {
    () => {
        deps!();
        # [expect (clippy :: panic)] fn populate_litemap < S > (map : & mut LiteMap < u32 , u64 , S >) where S : StoreMut < u32 , u64 > + Debug , { assert_eq ! (0 , map . len ()) ; assert ! (map . is_empty ()) ; for (k , v) in SORTED_DATA . iter () { if map . try_append (* k , * v) . is_some () { panic ! ("appending sorted data: {k:?} to {map:?}") ; } ; } assert_eq ! (10 , map . len ()) ; for (k , v) in RANDOM_DATA . iter () { match map . try_append (* k , * v) { Some (_) => () , None => panic ! ("cannot append random data: {k:?} to{map:?}") , } ; } assert_eq ! (10 , map . len ()) ; for (k , v) in RANDOM_DATA . iter () { map . insert (* k , * v) ; } assert_eq ! (20 , map . len ()) ; }
    };
}

populate_litemap!()