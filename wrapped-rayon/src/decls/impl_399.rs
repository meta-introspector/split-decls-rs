macro_rules! deps {
    () => {
        IntoIter!();
        Empty!();
        Producer!();
        EmptyProducer!();
        Folder!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl < T : Send > Producer for EmptyProducer < T > { type Item = T ; type IntoIter = std :: iter :: Empty < T > ; fn into_iter (self) -> Self :: IntoIter { std :: iter :: empty () } fn split_at (self , index : usize) -> (Self , Self) { debug_assert_eq ! (index , 0) ; (self , EmptyProducer (PhantomData)) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder } }
    };
}

impl_399!();