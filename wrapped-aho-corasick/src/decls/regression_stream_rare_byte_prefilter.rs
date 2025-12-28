macro_rules! deps {
    () => {
        AhoCorasick!();
    };
}

macro_rules! regression_stream_rare_byte_prefilter {
    () => {
        deps!();
        # [cfg (feature = "std")] # [test] fn regression_stream_rare_byte_prefilter () { use std :: io :: Read ; const MAGIC : [u8 ; 5] = * b"1234j" ; const BEGIN : usize = 65_535 ; # [doc = " This is just a structure that implements Reader. The reader"] # [doc = " implementation will simulate a file filled with 0, except for the MAGIC"] # [doc = " string at offset BEGIN."] # [derive (Default)] struct R { read : usize , } impl Read for R { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { if self . read > 100000 { return Ok (0) ; } let mut from = 0 ; if self . read < BEGIN { from = buf . len () . min (BEGIN - self . read) ; for x in 0 .. from { buf [x] = 0 ; } self . read += from ; } if self . read >= BEGIN && self . read <= BEGIN + MAGIC . len () { let to = buf . len () . min (BEGIN + MAGIC . len () - self . read + from) ; if to > from { buf [from .. to] . copy_from_slice (& MAGIC [self . read - BEGIN .. self . read - BEGIN + to - from] ,) ; self . read += to - from ; from = to ; } } for x in from .. buf . len () { buf [x] = 0 ; self . read += 1 ; } Ok (buf . len ()) } } fn run () -> std :: io :: Result < () > { let aut = AhoCorasick :: builder () . byte_classes (false) . build (& [& MAGIC]) . unwrap () ; let mut buf = alloc :: vec ! [] ; R :: default () . read_to_end (& mut buf) ? ; let from_whole = aut . find_iter (& buf) . next () . unwrap () . start () ; let mut file = std :: io :: BufReader :: new (R :: default ()) ; let begin = aut . stream_find_iter (& mut file) . next () . expect ("NOT FOUND!!!!") ? . start () ; assert_eq ! (from_whole , begin) ; Ok (()) } run () . unwrap () }
    };
}

regression_stream_rare_byte_prefilter!()