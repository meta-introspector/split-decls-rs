macro_rules! deps {
    () => {
        LenType!();
        SortedLinkedListInner!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl < T , Idx , K , S > Drop for SortedLinkedListInner < T , Idx , K , S > where Idx : LenType , S : SortedLinkedListStorage < T , Idx > + ? Sized , { fn drop (& mut self) { let mut index = self . head ; while let Some (i) = index . to_non_max () { let node = self . node_at_mut (i) ; index = node . next ; unsafe { ptr :: drop_in_place (node . val . as_mut_ptr ()) ; } } } }
    };
}

impl_447!()