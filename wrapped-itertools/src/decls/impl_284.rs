macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl < 'a , I > Iterator for Chunk < 'a , I > where I : Iterator , I :: Item : 'a , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if let elt @ Some (..) = self . first . take () { return elt ; } self . parent . step (self . index) } }
    };
}

impl_284!();