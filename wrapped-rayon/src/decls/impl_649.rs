macro_rules! deps {
    () => {
        IntersperseIter!();
        IntersperseProducer!();
        Producer!();
        IntoIter!();
        IntersperseFolder!();
        Folder!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        impl < P > Producer for IntersperseProducer < P > where P : Producer < Item : Clone + Send > , { type Item = P :: Item ; type IntoIter = IntersperseIter < P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { IntersperseIter { base : self . base . into_iter () . fuse () , item : self . item , clone_first : self . len > 0 && self . clone_first , clone_last : self . len > 1 && ((self . len & 1 == 0) ^ self . clone_first) , } } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { debug_assert ! (index <= self . len) ; let base_index = (index + ! self . clone_first as usize) / 2 ; let (left_base , right_base) = self . base . split_at (base_index) ; let left = IntersperseProducer { base : left_base , item : self . item . clone () , len : index , clone_first : self . clone_first , } ; let right = IntersperseProducer { base : right_base , item : self . item , len : self . len - index , clone_first : (index & 1 == 1) ^ self . clone_first , } ; (left , right) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { let folder1 = IntersperseFolder { base : folder , item : self . item , clone_first : self . clone_first , } ; self . base . fold_with (folder1) . base } }
    };
}

impl_649!();