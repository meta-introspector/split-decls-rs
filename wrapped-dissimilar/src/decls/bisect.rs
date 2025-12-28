macro_rules! deps {
    () => {
        Diff!();
        Range!();
    };
}

macro_rules! bisect {
    () => {
        deps!();
        fn bisect < 'a , 'b > (text1 : Range < 'a > , text2 : Range < 'b >) -> Vec < Diff < 'a , 'b > > { let max_d = (text1 . len + text2 . len + 1) / 2 ; let v_offset = max_d ; let v_len = 2 * max_d ; let mut v1 = vec ! [- 1isize ; v_len] ; let mut v2 = vec ! [- 1isize ; v_len] ; v1 [v_offset + 1] = 0 ; v2 [v_offset + 1] = 0 ; let delta = text1 . len as isize - text2 . len as isize ; let front = delta % 2 != 0 ; let mut k1start = 0 ; let mut k1end = 0 ; let mut k2start = 0 ; let mut k2end = 0 ; for d in 0 .. max_d as isize { let mut k1 = - d + k1start ; while k1 <= d - k1end { let k1_offset = (v_offset as isize + k1) as usize ; let mut x1 = if k1 == - d || (k1 != d && v1 [k1_offset - 1] < v1 [k1_offset + 1]) { v1 [k1_offset + 1] } else { v1 [k1_offset - 1] + 1 } as usize ; let mut y1 = (x1 as isize - k1) as usize ; if let (Some (s1) , Some (s2)) = (text1 . get (x1 ..) , text2 . get (y1 ..)) { let advance = common_prefix (s1 , s2) ; x1 += advance ; y1 += advance ; } v1 [k1_offset] = x1 as isize ; if x1 > text1 . len { k1end += 2 ; } else if y1 > text2 . len { k1start += 2 ; } else if front { let k2_offset = v_offset as isize + delta - k1 ; if k2_offset >= 0 && k2_offset < v_len as isize && v2 [k2_offset as usize] != - 1 { let x2 = text1 . len as isize - v2 [k2_offset as usize] ; if x1 as isize >= x2 { return bisect_split (text1 , text2 , x1 , y1) ; } } } k1 += 2 ; } let mut k2 = - d + k2start ; while k2 <= d - k2end { let k2_offset = (v_offset as isize + k2) as usize ; let mut x2 = if k2 == - d || (k2 != d && v2 [k2_offset - 1] < v2 [k2_offset + 1]) { v2 [k2_offset + 1] } else { v2 [k2_offset - 1] + 1 } as usize ; let mut y2 = (x2 as isize - k2) as usize ; if x2 < text1 . len && y2 < text2 . len { let advance = common_suffix (text1 . substring (.. text1 . len - x2) , text2 . substring (.. text2 . len - y2) ,) ; x2 += advance ; y2 += advance ; } v2 [k2_offset] = x2 as isize ; if x2 > text1 . len { k2end += 2 ; } else if y2 > text2 . len { k2start += 2 ; } else if ! front { let k1_offset = v_offset as isize + delta - k2 ; if k1_offset >= 0 && k1_offset < v_len as isize && v1 [k1_offset as usize] != - 1 { let x1 = v1 [k1_offset as usize] as usize ; let y1 = v_offset + x1 - k1_offset as usize ; x2 = text1 . len - x2 ; if x1 >= x2 { return bisect_split (text1 , text2 , x1 , y1) ; } } } k2 += 2 ; } } vec ! [Diff :: Delete (text1) , Diff :: Insert (text2)] }
    };
}

bisect!()