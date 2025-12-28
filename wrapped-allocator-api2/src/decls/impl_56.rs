macro_rules! deps {
    () => {
        BoxIter!();
        Allocator!();
        Box!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < I : Iterator + ? Sized , A : Allocator > BoxIter for Box < I , A > { type Item = I :: Item ; # [inline (always)] fn last (self) -> Option < I :: Item > { # [inline (always)] fn some < T > (_ : Option < T > , x : T) -> Option < T > { Some (x) } self . fold (None , some) } }
    };
}

impl_56!()