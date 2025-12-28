macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < T , const CAP : usize > Drop for IntoIter < T , CAP > { fn drop (& mut self) { let index = self . index ; let len = self . v . len () ; unsafe { self . v . set_len (0) ; let elements = slice :: from_raw_parts_mut (self . v . get_unchecked_ptr (index) , len - index) ; ptr :: drop_in_place (elements) ; } } }
    };
}

impl_55!();