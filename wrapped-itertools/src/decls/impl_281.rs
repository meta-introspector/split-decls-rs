macro_rules! deps {
    () => {
        Chunks!();
        Chunk!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl < 'a , I > Iterator for Chunks < 'a , I > where I : Iterator , I :: Item : 'a , { type Item = Chunk < 'a , I > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let index = self . parent . index . get () ; self . parent . index . set (index + 1) ; let inner = & mut * self . parent . inner . borrow_mut () ; inner . step (index) . map (| elt | Chunk { parent : self . parent , index , first : Some (elt) , }) } }
    };
}

impl_281!()