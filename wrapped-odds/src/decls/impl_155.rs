macro_rules! deps {
    () => {
        VecFindRemove!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < T > VecFindRemove for Vec < T > { type Item = T ; fn find_remove < U > (& mut self , elt : & U) -> Option < (usize , Self :: Item) > where Self :: Item : PartialEq < U > , { self . find (elt) . map (| i | (i , self . remove (i))) } fn rfind_remove < U > (& mut self , elt : & U) -> Option < (usize , Self :: Item) > where Self :: Item : PartialEq < U > , { self . rfind (elt) . map (| i | (i , self . remove (i))) } }
    };
}

impl_155!()