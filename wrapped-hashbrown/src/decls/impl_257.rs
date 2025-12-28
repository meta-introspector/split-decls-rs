macro_rules! deps {
    () => {
        IntoKeys!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < K , V , A : Allocator > Iterator for IntoKeys < K , V , A > { type Item = K ; # [inline] fn next (& mut self) -> Option < K > { self . inner . next () . map (| (k , _) | k) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn fold < B , F > (self , init : B , mut f : F) -> B where Self : Sized , F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , | acc , (k , _) | f (acc , k)) } }
    };
}

impl_257!();