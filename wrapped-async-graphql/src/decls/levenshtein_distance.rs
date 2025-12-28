macro_rules! levenshtein_distance {
    () => {
        fn levenshtein_distance (s1 : & str , s2 : & str) -> usize { let mut column : Vec < _ > = (0 ..= s1 . len ()) . collect () ; for (x , rx) in s2 . bytes () . enumerate () { column [0] = x + 1 ; let mut lastdiag = x ; for (y , ry) in s1 . bytes () . enumerate () { let olddiag = column [y + 1] ; if rx != ry { lastdiag += 1 ; } column [y + 1] = (column [y + 1] + 1) . min ((column [y] + 1) . min (lastdiag)) ; lastdiag = olddiag ; } } column [s1 . len ()] }
    };
}

levenshtein_distance!();