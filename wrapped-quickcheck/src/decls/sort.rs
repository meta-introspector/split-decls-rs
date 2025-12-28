macro_rules! sort {
    () => {
        # [test] fn sort () { fn prop (mut xs : Vec < isize >) -> bool { xs . sort_unstable () ; for i in xs . windows (2) { if i [0] > i [1] { return false ; } } true } quickcheck (prop as fn (Vec < isize >) -> bool) ; }
    };
}

sort!();