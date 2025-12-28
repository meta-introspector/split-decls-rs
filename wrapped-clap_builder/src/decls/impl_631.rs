macro_rules! deps {
    () => {
        FlatSet!();
        Iter!();
    };
}

macro_rules! impl_631 {
    () => {
        deps!();
        impl < T : PartialEq + Eq > FlatSet < T > { pub (crate) fn new () -> Self { Default :: default () } pub (crate) fn insert (& mut self , value : T) -> bool { for existing in & self . inner { if * existing == value { return false ; } } self . inner . push (value) ; true } pub (crate) fn contains < Q : ? Sized > (& self , value : & Q) -> bool where T : Borrow < Q > , Q : Eq , { for existing in & self . inner { if existing . borrow () == value { return true ; } } false } pub (crate) fn retain < F > (& mut self , f : F) where F : FnMut (& T) -> bool , { self . inner . retain (f) ; } pub (crate) fn is_empty (& self) -> bool { self . inner . is_empty () } pub (crate) fn iter (& self) -> std :: slice :: Iter < '_ , T > { self . inner . iter () } pub (crate) fn sort_by_key < K , F > (& mut self , f : F) where F : FnMut (& T) -> K , K : Ord , { self . inner . sort_by_key (f) ; } }
    };
}

impl_631!()