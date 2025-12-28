macro_rules! deps {
    () => {
        Groups!();
        Group!();
    };
}

macro_rules! impl_268 {
    () => {
        deps!();
        impl < 'a , K , I , F > Iterator for Groups < 'a , K , I , F > where I : Iterator , I :: Item : 'a , F : FnMut (& I :: Item) -> K , K : PartialEq , { type Item = (K , Group < 'a , K , I , F >) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let index = self . parent . index . get () ; self . parent . index . set (index + 1) ; let inner = & mut * self . parent . inner . borrow_mut () ; inner . step (index) . map (| elt | { let key = inner . group_key (index) ; (key , Group { parent : self . parent , index , first : Some (elt) , } ,) }) } }
    };
}

impl_268!();