macro_rules! deps {
    () => {
        SlotsIter!();
        Slots!();
        NFA!();
        NonMaxUsize!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Slots { const LIMIT : usize = 32 ; # [doc = " Insert the slot at the given bit index."] fn insert (self , slot : usize) -> Slots { debug_assert ! (slot < Slots :: LIMIT) ; Slots (self . 0 | (1 << slot . as_u32 ())) } # [doc = " Remove the slot at the given bit index."] fn remove (self , slot : usize) -> Slots { debug_assert ! (slot < Slots :: LIMIT) ; Slots (self . 0 & ! (1 << slot . as_u32 ())) } # [doc = " Returns true if and only if this set contains no slots."] fn is_empty (self) -> bool { self . 0 == 0 } # [doc = " Returns an iterator over all of the set bits in this set."] fn iter (self) -> SlotsIter { SlotsIter { slots : self } } # [doc = " For the position `at` in the current haystack, copy it to"] # [doc = " `caller_explicit_slots` for all slots that are in this set."] # [doc = ""] # [doc = " Callers may pass a slice of any length. Slots in this set bigger than"] # [doc = " the length of the given explicit slots are simply skipped."] # [doc = ""] # [doc = " The slice *must* correspond only to the explicit slots and the first"] # [doc = " element of the slice must always correspond to the first explicit slot"] # [doc = " in the corresponding NFA."] fn apply (self , at : usize , caller_explicit_slots : & mut [Option < NonMaxUsize >] ,) { if self . is_empty () { return ; } let at = NonMaxUsize :: new (at) ; for slot in self . iter () { if slot >= caller_explicit_slots . len () { break ; } caller_explicit_slots [slot] = at ; } } }
    };
}

impl_90!();