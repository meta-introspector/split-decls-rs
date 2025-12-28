macro_rules! deps {
    () => {
        SliceFind!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < T > SliceFind for [T] { type Item = T ; fn find < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { SliceIter :: from (self) . position (move | x | * x == * elt) } fn rfind < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { SliceIter :: from (self) . rposition (move | x | * x == * elt) } }
    };
}

impl_86!();