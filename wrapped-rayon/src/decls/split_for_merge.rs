macro_rules! split_for_merge {
    () => {
        # [doc = " Splits two sorted slices so that they can be merged in parallel."] # [doc = ""] # [doc = " Returns two indices `(a, b)` so that slices `left[..a]` and `right[..b]` come before"] # [doc = " `left[a..]` and `right[b..]`."] fn split_for_merge < T , F > (left : & [T] , right : & [T] , is_less : & F) -> (usize , usize) where F : Fn (& T , & T) -> bool , { let left_len = left . len () ; let right_len = right . len () ; if left_len >= right_len { let left_mid = left_len / 2 ; let mut a = 0 ; let mut b = right_len ; while a < b { let m = a + (b - a) / 2 ; if is_less (& right [m] , & left [left_mid]) { a = m + 1 ; } else { b = m ; } } (left_mid , a) } else { let right_mid = right_len / 2 ; let mut a = 0 ; let mut b = left_len ; while a < b { let m = a + (b - a) / 2 ; if is_less (& right [right_mid] , & left [m]) { b = m ; } else { a = m + 1 ; } } (a , right_mid) } }
    };
}

split_for_merge!()