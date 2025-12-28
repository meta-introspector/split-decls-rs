macro_rules! deps {
    () => {
        SliceFind!();
        RevSlice!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T > SliceFind for RevSlice < T > { type Item = T ; fn find < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { self . 0 . rfind (elt) . map (move | i | self . raw_index_no_wrap (i)) } fn rfind < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { self . 0 . find (elt) . map (move | i | self . raw_index_no_wrap (i)) } }
    };
}

impl_65!();