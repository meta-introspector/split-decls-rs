macro_rules! deps {
    () => {
        RwLockWriteGuardDetached!();
        RefMutMulti!();
        IterMut!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash + 'a , V : 'a > Iterator for IterMut < 'a , K , V > { type Item = RefMutMulti < 'a , K , V > ; fn next (& mut self) -> Option < Self :: Item > { loop { if let Some (current) = self . current . as_mut () { if let Some ((k , v)) = current . 1 . next () { let guard = current . 0 . clone () ; return Some (RefMutMulti :: new (guard , k , v)) ; } } let guard = self . shards . next () ? . write () ; let (guard , shard) = unsafe { RwLockWriteGuardDetached :: detach_from (guard) } ; let iter = shard . iter_mut () ; self . current = Some ((Arc :: new (guard) , iter)) ; } } }
    };
}

impl_14!();