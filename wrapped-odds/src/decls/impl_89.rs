macro_rules! deps {
    () => {
        SliceFindSplit!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < T > SliceFindSplit for [T] { type Item = T ; fn find_split < U : ? Sized > (& self , elt : & U) -> (& Self , & Self) where Self :: Item : PartialEq < U > , { let i = self . find (elt) . unwrap_or (self . len ()) ; unsafe { split_at_unchecked (self , i) } } fn find_split_mut < U : ? Sized > (& mut self , elt : & U) -> (& mut Self , & mut Self) where Self :: Item : PartialEq < U > , { let i = self . find (elt) . unwrap_or (self . len ()) ; self . split_at_mut (i) } fn rfind_split < U : ? Sized > (& self , elt : & U) -> (& Self , & Self) where Self :: Item : PartialEq < U > , { let i = self . rfind (elt) . unwrap_or (0) ; unsafe { split_at_unchecked (self , i) } } fn rfind_split_mut < U : ? Sized > (& mut self , elt : & U) -> (& mut Self , & mut Self) where Self :: Item : PartialEq < U > , { let i = self . rfind (elt) . unwrap_or (0) ; self . split_at_mut (i) } }
    };
}

impl_89!()