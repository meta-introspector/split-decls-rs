macro_rules! deps {
    () => {
        ExtractIf!();
    };
}

macro_rules! impl_527 {
    () => {
        deps!();
        impl < T , F , A : Allocator > Iterator for ExtractIf < '_ , T , F , A > where F : FnMut (& mut T) -> bool , { type Item = T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . inner . next (| val | (self . f) (val)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . inner . iter . size_hint () . 1) } }
    };
}

impl_527!();