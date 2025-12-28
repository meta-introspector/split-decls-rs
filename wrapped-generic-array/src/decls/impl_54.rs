macro_rules! deps {
    () => {
        GenericArrayIter!();
        ArrayLength!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < T , N : ArrayLength > DoubleEndedIterator for GenericArrayIter < T , N > { # [inline] fn next_back (& mut self) -> Option < T > { if self . index < self . index_back { self . index_back -= 1 ; unsafe { Some (ptr :: read (self . array . get_unchecked (self . index_back))) } } else { None } } # [inline] fn rfold < B , F > (mut self , init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let ret = unsafe { let GenericArrayIter { ref array , index , ref mut index_back , } = self ; let remaining = array . get_unchecked (index .. * index_back) ; remaining . iter () . rfold (init , | acc , src | { let value = ptr :: read (src) ; * index_back -= 1 ; f (acc , value) }) } ; mem :: forget (self) ; ret } fn nth_back (& mut self , n : usize) -> Option < T > { let next_back = self . index_back - cmp :: min (n , self . len ()) ; unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (next_back .. self . index_back)) ; } self . index_back = next_back ; self . next_back () } }
    };
}

impl_54!()