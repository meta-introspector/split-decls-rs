macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_xof_seek {
    () => {
        deps!();
        # [test] fn test_xof_seek () { let mut out = [0 ; 533] ; let mut hasher = crate :: Hasher :: new () ; hasher . update (b"foo") ; hasher . finalize_xof () . fill (& mut out) ; assert_eq ! (hasher . finalize () . as_bytes () , & out [0 .. 32]) ; let mut reader = hasher . finalize_xof () ; reader . set_position (303) ; let mut out2 = [0 ; 102] ; reader . fill (& mut out2) ; assert_eq ! (& out [303 ..] [.. 102] , & out2 [..]) ; # [cfg (feature = "std")] { use std :: io :: prelude :: * ; let mut reader = hasher . finalize_xof () ; reader . seek (std :: io :: SeekFrom :: Start (303)) . unwrap () ; let mut out3 = Vec :: new () ; reader . by_ref () . take (102) . read_to_end (& mut out3) . unwrap () ; assert_eq ! (& out [303 ..] [.. 102] , & out3 [..]) ; assert_eq ! (reader . seek (std :: io :: SeekFrom :: Current (0)) . unwrap () , 303 + 102) ; reader . seek (std :: io :: SeekFrom :: Current (- 5)) . unwrap () ; assert_eq ! (reader . seek (std :: io :: SeekFrom :: Current (0)) . unwrap () , 303 + 102 - 5) ; let mut out4 = [0 ; 17] ; assert_eq ! (reader . read (& mut out4) . unwrap () , 17) ; assert_eq ! (& out [303 + 102 - 5 ..] [.. 17] , & out4 [..]) ; assert_eq ! (reader . seek (std :: io :: SeekFrom :: Current (0)) . unwrap () , 303 + 102 - 5 + 17) ; assert ! (reader . seek (std :: io :: SeekFrom :: End (0)) . is_err ()) ; assert ! (reader . seek (std :: io :: SeekFrom :: Current (- 1000)) . is_err ()) ; } }
    };
}

test_xof_seek!();