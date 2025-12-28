macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! test_update_reader_interrupted {
    () => {
        deps!();
        # [test] # [cfg (feature = "std")] fn test_update_reader_interrupted () -> std :: io :: Result < () > { use std :: io ; struct InterruptingReader < 'a > { already_interrupted : bool , slice : & 'a [u8] , } impl < 'a > InterruptingReader < 'a > { fn new (slice : & 'a [u8]) -> Self { Self { already_interrupted : false , slice , } } } impl < 'a > io :: Read for InterruptingReader < 'a > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if ! self . already_interrupted { self . already_interrupted = true ; return Err (io :: Error :: from (io :: ErrorKind :: Interrupted)) ; } let take = std :: cmp :: min (self . slice . len () , buf . len ()) ; buf [.. take] . copy_from_slice (& self . slice [.. take]) ; self . slice = & self . slice [take ..] ; Ok (take) } } let input = b"hello world" ; let mut reader = InterruptingReader :: new (input) ; let mut hasher = crate :: Hasher :: new () ; hasher . update_reader (& mut reader) ? ; assert_eq ! (hasher . finalize () , crate :: hash (input)) ; Ok (()) }
    };
}

test_update_reader_interrupted!()