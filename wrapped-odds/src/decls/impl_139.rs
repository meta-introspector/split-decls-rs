macro_rules! deps {
    () => {
        CharWindows!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'a > CharWindows < 'a > { fn new (s : & 'a str , n : usize) -> Self { assert ! (n != 0) ; match s . char_indices () . nth (n - 1) { None => CharWindows { s : s , a : s . len () , b : s . len () , } , Some ((i , ch)) => CharWindows { s : s , a : 0 , b : i + ch . len_utf8 () , } , } } }
    };
}

impl_139!()