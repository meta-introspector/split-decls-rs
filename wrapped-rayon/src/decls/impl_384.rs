macro_rules! deps {
    () => {
        Producer!();
        Copied!();
        CopiedFolder!();
        Folder!();
        CopiedProducer!();
        IntoIter!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < 'a , T , P > Producer for CopiedProducer < P > where P : Producer < Item = & 'a T > , T : 'a + Copy , { type Item = T ; type IntoIter = iter :: Copied < P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () . copied () } fn min_len (& self) -> usize { self . base . min_len () } fn max_len (& self) -> usize { self . base . max_len () } fn split_at (self , index : usize) -> (Self , Self) { let (left , right) = self . base . split_at (index) ; (CopiedProducer { base : left } , CopiedProducer { base : right } ,) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . base . fold_with (CopiedFolder { base : folder }) . base } }
    };
}

impl_384!()