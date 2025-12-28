macro_rules! deps {
    () => {
        SparseSets!();
        SparseSet!();
    };
}

macro_rules! impl_941 {
    () => {
        deps!();
        impl SparseSets { # [doc = " Create a new pair of sparse sets where each set has the given capacity."] # [doc = ""] # [doc = " This panics if the capacity given is bigger than `StateID::LIMIT`."] pub (crate) fn new (capacity : usize) -> SparseSets { SparseSets { set1 : SparseSet :: new (capacity) , set2 : SparseSet :: new (capacity) , } } # [doc = " Resizes these sparse sets to have the new capacity given."] # [doc = ""] # [doc = " The sets are automatically cleared."] # [doc = ""] # [doc = " This panics if the capacity given is bigger than `StateID::LIMIT`."] # [inline] pub (crate) fn resize (& mut self , new_capacity : usize) { self . set1 . resize (new_capacity) ; self . set2 . resize (new_capacity) ; } # [doc = " Clear both sparse sets."] pub (crate) fn clear (& mut self) { self . set1 . clear () ; self . set2 . clear () ; } # [doc = " Swap set1 with set2."] pub (crate) fn swap (& mut self) { core :: mem :: swap (& mut self . set1 , & mut self . set2) ; } # [doc = " Returns the memory usage, in bytes, used by this pair of sparse sets."] pub (crate) fn memory_usage (& self) -> usize { self . set1 . memory_usage () + self . set2 . memory_usage () } }
    };
}

impl_941!();