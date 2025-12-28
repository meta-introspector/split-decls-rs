macro_rules! deps {
    () => {
        Idx!();
        IdxRange!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl < T > Iterator for IdxRange < T > { type Item = Idx < T > ; fn next (& mut self) -> Option < Self :: Item > { self . range . next () . map (| raw | Idx :: from_raw (raw . into ())) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } fn count (self) -> usize where Self : Sized , { self . range . count () } fn last (self) -> Option < Self :: Item > where Self : Sized , { self . range . last () . map (| raw | Idx :: from_raw (raw . into ())) } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . range . nth (n) . map (| raw | Idx :: from_raw (raw . into ())) } }
    };
}

impl_38!();