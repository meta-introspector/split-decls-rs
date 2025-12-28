macro_rules! deps {
    () => {
        Folder!();
        Fissile!();
        SplitProducer!();
    };
}

macro_rules! impl_1270 {
    () => {
        deps!();
        # [doc = " Implement support for `SplitProducer`."] impl < T , P > Fissile < P > for & [T] where P : Fn (& T) -> bool , { fn length (& self) -> usize { self . len () } fn midpoint (& self , end : usize) -> usize { end / 2 } fn find (& self , separator : & P , start : usize , end : usize) -> Option < usize > { self [start .. end] . iter () . position (separator) } fn rfind (& self , separator : & P , end : usize) -> Option < usize > { self [.. end] . iter () . rposition (separator) } fn split_once < const INCL : bool > (self , index : usize) -> (Self , Self) { if INCL { self . split_at (index + 1) } else { let (left , right) = self . split_at (index) ; (left , & right [1 ..]) } } fn fold_splits < F , const INCL : bool > (self , separator : & P , folder : F , skip_last : bool) -> F where F : Folder < Self > , Self : Send , { if INCL { debug_assert ! (! skip_last) ; folder . consume_iter (self . split_inclusive (separator)) } else { let mut split = self . split (separator) ; if skip_last { split . next_back () ; } folder . consume_iter (split) } } }
    };
}

impl_1270!();