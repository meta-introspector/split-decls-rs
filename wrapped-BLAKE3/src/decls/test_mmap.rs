macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_mmap {
    () => {
        deps!();
        # [test] # [cfg (feature = "mmap")] # [cfg (not (miri))] fn test_mmap () -> Result < () , std :: io :: Error > { use std :: io :: prelude :: * ; let mut input = vec ! [0 ; 1_000_000] ; paint_test_input (& mut input) ; let mut tempfile = tempfile :: NamedTempFile :: new () ? ; tempfile . write_all (& input) ? ; tempfile . flush () ? ; assert_eq ! (crate :: Hasher :: new () . update_mmap (tempfile . path ()) ? . finalize () , crate :: hash (& input) ,) ; Ok (()) }
    };
}

test_mmap!();