macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn smoke_test () { Symbol :: intern ("isize") ; let base_len = MAP . get () . unwrap () . len () ; let hello = Symbol :: intern ("hello") ; let world = Symbol :: intern ("world") ; let more_worlds = world . clone () ; let bang = Symbol :: intern ("!") ; let q = Symbol :: intern ("?") ; assert_eq ! (MAP . get () . unwrap () . len () , base_len + 4) ; let bang2 = Symbol :: intern ("!") ; assert_eq ! (MAP . get () . unwrap () . len () , base_len + 4) ; drop (bang2) ; assert_eq ! (MAP . get () . unwrap () . len () , base_len + 4) ; drop (q) ; assert_eq ! (MAP . get () . unwrap () . len () , base_len + 3) ; let default = Symbol :: intern ("default") ; let many_worlds = world . clone () ; assert_eq ! (MAP . get () . unwrap () . len () , base_len + 3) ; assert_eq ! ("hello default world!" , format ! ("{} {} {}{}" , hello . as_str () , default . as_str () , world . as_str () , bang . as_str ())) ; drop (default) ; assert_eq ! ("hello world!" , format ! ("{} {}{}" , hello . as_str () , world . as_str () , bang . as_str ())) ; drop (many_worlds) ; drop (more_worlds) ; drop (hello) ; drop (world) ; drop (bang) ; assert_eq ! (MAP . get () . unwrap () . len () , base_len) ; } }
    };
}

tests!()