macro_rules! deps {
    () => {
        Range!();
        Diff!();
    };
}

macro_rules! compute {
    () => {
        deps!();
        fn compute < 'a , 'b > (text1 : Range < 'a > , text2 : Range < 'b >) -> Vec < Diff < 'a , 'b > > { match (text1 . is_empty () , text2 . is_empty ()) { (true , true) => return Vec :: new () , (true , false) => return vec ! [Diff :: Insert (text2)] , (false , true) => return vec ! [Diff :: Delete (text1)] , (false , false) => { } } if text1 . len > text2 . len { if let Some (i) = text1 . find (text2) { return vec ! [Diff :: Delete (text1 . substring (.. i)) , Diff :: Equal (text1 . substring (i .. i + text2 . len) , text2) , Diff :: Delete (text1 . substring (i + text2 . len ..)) ,] ; } } else { if let Some (i) = text2 . find (text1) { return vec ! [Diff :: Insert (text2 . substring (.. i)) , Diff :: Equal (text1 , text2 . substring (i .. i + text1 . len)) , Diff :: Insert (text2 . substring (i + text1 . len ..)) ,] ; } } if text1 . len == 1 || text2 . len == 1 { return vec ! [Diff :: Delete (text1) , Diff :: Insert (text2)] ; } bisect (text1 , text2) }
    };
}

compute!();