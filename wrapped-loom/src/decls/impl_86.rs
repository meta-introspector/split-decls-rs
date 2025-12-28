macro_rules! deps {
    () => {
        Ref!();
        Object!();
        Store!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < T > Store < T > { # [doc = " Create a new, empty, object store"] pub (super) fn with_capacity (capacity : usize) -> Store < T > { Store { entries : Vec :: with_capacity (capacity) , } } pub (super) fn len (& self) -> usize { self . entries . len () } pub (super) fn capacity (& self) -> usize { self . entries . capacity () } pub (super) fn reserve_exact (& mut self , additional : usize) { self . entries . reserve_exact (additional) ; } # [doc = " Insert an object into the store"] pub (super) fn insert < O > (& mut self , item : O) -> Ref < O > where O : Object < Entry = T > , { let index = self . entries . len () ; self . entries . push (item . into_entry ()) ; Ref { index , _p : PhantomData , } } pub (crate) fn truncate < O > (& mut self , obj : Ref < O >) { let target = obj . index + 1 ; self . entries . truncate (target) ; } pub (crate) fn clear (& mut self) { self . entries . clear () ; } pub (super) fn iter_ref < O > (& self) -> impl DoubleEndedIterator < Item = Ref < O > > + '_ where O : Object < Entry = T > , { self . entries . iter () . enumerate () . filter (| (_ , e) | O :: get_ref (e) . is_some ()) . map (| (index , _) | Ref { index , _p : PhantomData , }) } pub (super) fn iter_mut < 'a , O > (& 'a mut self) -> impl DoubleEndedIterator < Item = & 'a mut O > where O : Object < Entry = T > + 'a , { self . entries . iter_mut () . filter_map (O :: get_mut) } }
    };
}

impl_86!();