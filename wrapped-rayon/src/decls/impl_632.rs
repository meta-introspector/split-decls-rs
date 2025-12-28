macro_rules! deps {
    () => {
        IntoIter!();
        InterleaveProducer!();
        Producer!();
        InterleaveSeq!();
    };
}

macro_rules! impl_632 {
    () => {
        deps!();
        impl < I , J > Producer for InterleaveProducer < I , J > where I : Producer , J : Producer < Item = I :: Item > , { type Item = I :: Item ; type IntoIter = InterleaveSeq < I :: IntoIter , J :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { InterleaveSeq { i : self . i . into_iter () . fuse () , j : self . j . into_iter () . fuse () , i_next : self . i_next , } } fn min_len (& self) -> usize { Ord :: max (self . i . min_len () , self . j . min_len ()) } fn max_len (& self) -> usize { Ord :: min (self . i . max_len () , self . j . max_len ()) } # [doc = " We know 0 < index <= self.i_len + self.j_len"] # [doc = ""] # [doc = " Find a, b satisfying:"] # [doc = ""] # [doc = "  (1) 0 < a <= self.i_len"] # [doc = "  (2) 0 < b <= self.j_len"] # [doc = "  (3) a + b == index"] # [doc = ""] # [doc = " For even splits, set a = b = index/2."] # [doc = " For odd splits, set a = (index/2)+1, b = index/2, if `i`"] # [doc = " should yield the next element, otherwise, if `j` should yield"] # [doc = " the next element, set a = index/2 and b = (index/2)+1"] fn split_at (self , index : usize) -> (Self , Self) { # [inline] fn odd_offset (flag : bool) -> usize { (! flag) as usize } let even = index % 2 == 0 ; let idx = index >> 1 ; let (i_idx , j_idx) = (idx + odd_offset (even || self . i_next) , idx + odd_offset (even || ! self . i_next) ,) ; let (i_split , j_split) = if self . i_len >= i_idx && self . j_len >= j_idx { (i_idx , j_idx) } else if self . i_len >= i_idx { (index - self . j_len , self . j_len) } else { (self . i_len , index - self . i_len) } ; let trailing_i_next = even == self . i_next ; let (i_left , i_right) = self . i . split_at (i_split) ; let (j_left , j_right) = self . j . split_at (j_split) ; (InterleaveProducer :: new (i_left , j_left , i_split , j_split , self . i_next) , InterleaveProducer :: new (i_right , j_right , self . i_len - i_split , self . j_len - j_split , trailing_i_next ,) ,) } }
    };
}

impl_632!()