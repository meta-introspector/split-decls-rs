macro_rules! deps {
    () => {
        RawExtractIf!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < T , A : Allocator > RawExtractIf < '_ , T , A > { # [cfg_attr (feature = "inline-more" , inline)] pub (crate) fn next < F > (& mut self , mut f : F) -> Option < T > where F : FnMut (& mut T) -> bool , { unsafe { for item in & mut self . iter { if f (item . as_mut ()) { return Some (self . table . remove (item) . 0) ; } } } None } }
    };
}

impl_114!()