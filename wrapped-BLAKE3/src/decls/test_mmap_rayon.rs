macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_mmap_rayon {
    () => {
        deps!();
        # [test] # [cfg (feature = "mmap")] # [cfg (feature = "rayon")] # [cfg (not (miri))] fn test_mmap_rayon () -> Result < () , std :: io :: Error > { use std :: io :: prelude :: * ; let mut input = vec ! [0 ; 1_000_000] ; paint_test_input (& mut input) ; let mut tempfile = tempfile :: NamedTempFile :: new () ? ; tempfile . write_all (& input) ? ; tempfile . flush () ? ; assert_eq ! (crate :: Hasher :: new () . update_mmap_rayon (tempfile . path ()) ? . finalize () , crate :: hash (& input) ,) ; Ok (()) }
    };
}

test_mmap_rayon!();