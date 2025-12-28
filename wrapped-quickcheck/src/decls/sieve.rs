macro_rules! sieve {
    () => {
        fn sieve (n : usize) -> Vec < usize > { if n <= 1 { return vec ! [] ; } let mut marked = vec ! [false ; n + 1] ; marked [0] = true ; marked [1] = true ; marked [2] = true ; for p in 2 .. n { for i in (2 * p .. n) . filter (| & n | n % p == 0) { marked [i] = true ; } } marked . iter () . enumerate () . filter_map (| (i , & m) | if m { None } else { Some (i) }) . collect () }
    };
}

sieve!();