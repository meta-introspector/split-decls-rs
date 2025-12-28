macro_rules! deps {
    () => {
        Stdio!();
    };
}

macro_rules! test {
    () => {
        deps!();
        # [cfg (test)] mod test { use std :: fs :: OpenOptions ; use std :: io :: { Seek , Write } ; use super :: Stdio ; use super :: imp :: ReplacementGuard ; # [test] fn stdin () { let tempdir = snapbox :: dir :: DirRoot :: mutable_temp () . unwrap () ; let file = tempdir . path () . unwrap () . join ("stdin") ; let mut file = OpenOptions :: new () . read (true) . write (true) . create (true) . open (file) . unwrap () ; writeln ! (& mut file , "hello") . unwrap () ; file . seek (std :: io :: SeekFrom :: Start (0)) . unwrap () ; { let _guard = ReplacementGuard :: new (Stdio :: Stdin , & mut file) . unwrap () ; let line = std :: io :: stdin () . lines () . next () . unwrap () . unwrap () ; assert_eq ! (line , "hello") ; } } }
    };
}

test!();