macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_522 {
    () => {
        deps!();
        impl < T , A : Allocator > Iterator for Drain < '_ , T , A > { type Item = T ; fn next (& mut self) -> Option < T > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } fn fold < B , F > (self , init : B , f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , f) } }
    };
}

impl_522!()