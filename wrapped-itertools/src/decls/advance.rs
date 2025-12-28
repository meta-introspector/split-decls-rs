macro_rules! advance {
    () => {
        fn advance (indices : & mut [usize] , cycles : & mut [usize]) -> bool { let n = indices . len () ; let k = cycles . len () ; for i in (0 .. k) . rev () { if cycles [i] == 0 { cycles [i] = n - i - 1 ; indices [i ..] . rotate_left (1) ; } else { let swap_index = n - cycles [i] ; indices . swap (i , swap_index) ; cycles [i] -= 1 ; return false ; } } true }
    };
}

advance!();