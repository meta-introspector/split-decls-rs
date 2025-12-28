macro_rules! deps {
    () => {
        Positions!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < I , F > Iterator for Positions < I , F > where I : Iterator , F : FnMut (I :: Item) -> bool , { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { let f = & mut self . f ; self . iter . find_map (| (count , val) | f (val) . then_some (count)) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , self . iter . size_hint () . 1) } fn fold < B , G > (self , init : B , mut func : G) -> B where G : FnMut (B , Self :: Item) -> B , { let mut f = self . f ; self . iter . fold (init , | mut acc , (count , val) | { if f (val) { acc = func (acc , count) ; } acc }) } }
    };
}

impl_136!()