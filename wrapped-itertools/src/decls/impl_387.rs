macro_rules! deps {
    () => {
        PadUsing!();
    };
}

macro_rules! impl_387 {
    () => {
        deps!();
        impl < I , F > Iterator for PadUsing < I , F > where I : Iterator , F : FnMut (usize) -> I :: Item , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { match self . iter . next () { None => { if self . pos < self . min { let e = Some ((self . filler) (self . pos)) ; self . pos += 1 ; e } else { None } } e => { self . pos += 1 ; e } } } fn size_hint (& self) -> (usize , Option < usize >) { let tail = self . min . saturating_sub (self . pos) ; size_hint :: max (self . iter . size_hint () , (tail , Some (tail))) } fn fold < B , G > (self , mut init : B , mut f : G) -> B where G : FnMut (B , Self :: Item) -> B , { let mut pos = self . pos ; init = self . iter . fold (init , | acc , item | { pos += 1 ; f (acc , item) }) ; (pos .. self . min) . map (self . filler) . fold (init , f) } }
    };
}

impl_387!();