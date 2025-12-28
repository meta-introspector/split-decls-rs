macro_rules! deps {
    () => {
        Folder!();
        Fissile!();
        SplitProducer!();
    };
}

macro_rules! impl_1311 {
    () => {
        deps!();
        # [doc = " Implement support for `SplitProducer`."] impl < P : Pattern > Fissile < P > for & str { fn length (& self) -> usize { self . len () } fn midpoint (& self , end : usize) -> usize { find_char_midpoint (& self [.. end]) } fn find (& self , separator : & P , start : usize , end : usize) -> Option < usize > { separator . find_in (& self [start .. end]) } fn rfind (& self , separator : & P , end : usize) -> Option < usize > { separator . rfind_in (& self [.. end]) } fn split_once < const INCL : bool > (self , index : usize) -> (Self , Self) { if INCL { let separator = self [index ..] . chars () . next () . unwrap () ; self . split_at (index + separator . len_utf8 ()) } else { let (left , right) = self . split_at (index) ; let mut right_iter = right . chars () ; right_iter . next () ; (left , right_iter . as_str ()) } } fn fold_splits < F , const INCL : bool > (self , separator : & P , folder : F , skip_last : bool) -> F where F : Folder < Self > , { if INCL { debug_assert ! (! skip_last) ; separator . fold_inclusive_splits (self , folder) } else { separator . fold_splits (self , folder , skip_last) } } }
    };
}

impl_1311!();