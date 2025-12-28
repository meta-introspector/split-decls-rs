macro_rules! deps {
    () => {
        Drain!();
        IterMut!();
        ValuesMut!();
        DropFilteredValues!();
        ValueLinks!();
        Values!();
        Iter!();
        LinkedHashMap!();
        Keys!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < K , V , S > LinkedHashMap < K , V , S > { # [inline] pub fn with_hasher (hash_builder : S) -> Self { Self { hash_builder , table : HashTable :: new () , values : None , free : None , } } # [inline] pub fn with_capacity_and_hasher (capacity : usize , hash_builder : S) -> Self { Self { hash_builder , table : HashTable :: with_capacity (capacity) , values : None , free : None , } } # [inline] pub fn len (& self) -> usize { self . table . len () } # [inline] pub fn is_empty (& self) -> bool { self . len () == 0 } # [inline] pub fn clear (& mut self) { self . table . clear () ; if let Some (mut values) = self . values { unsafe { drop_value_nodes (values) ; values . as_mut () . links . value = ValueLinks { prev : values , next : values , } ; } } } # [inline] pub fn iter (& self) -> Iter < '_ , K , V > { let (head , tail) = if let Some (values) = self . values { unsafe { let ValueLinks { next , prev } = values . as_ref () . links . value ; (next . as_ptr () , prev . as_ptr ()) } } else { (ptr :: null_mut () , ptr :: null_mut ()) } ; Iter { head , tail , remaining : self . len () , marker : PhantomData , } } # [inline] pub fn iter_mut (& mut self) -> IterMut < '_ , K , V > { let (head , tail) = if let Some (values) = self . values { unsafe { let ValueLinks { next , prev } = values . as_ref () . links . value ; (Some (next) , Some (prev)) } } else { (None , None) } ; IterMut { head , tail , remaining : self . len () , marker : PhantomData , } } # [inline] pub fn drain (& mut self) -> Drain < '_ , K , V > { unsafe { let (head , tail) = if let Some (mut values) = self . values { let ValueLinks { next , prev } = values . as_ref () . links . value ; values . as_mut () . links . value = ValueLinks { next : values , prev : values , } ; (Some (next) , Some (prev)) } else { (None , None) } ; let len = self . len () ; self . table . clear () ; Drain { free : (& mut self . free) . into () , head , tail , remaining : len , marker : PhantomData , } } } # [inline] pub fn keys (& self) -> Keys < '_ , K , V > { Keys { inner : self . iter () } } # [inline] pub fn values (& self) -> Values < '_ , K , V > { Values { inner : self . iter () } } # [inline] pub fn values_mut (& mut self) -> ValuesMut < '_ , K , V > { ValuesMut { inner : self . iter_mut () , } } # [inline] pub fn front (& self) -> Option < (& K , & V) > { if self . is_empty () { return None ; } unsafe { let front = (* self . values . as_ptr ()) . links . value . next . as_ptr () ; let (key , value) = (* front) . entry_ref () ; Some ((key , value)) } } # [inline] pub fn back (& self) -> Option < (& K , & V) > { if self . is_empty () { return None ; } unsafe { let back = & * (* self . values . as_ptr ()) . links . value . prev . as_ptr () ; let (key , value) = (* back) . entry_ref () ; Some ((key , value)) } } # [inline] pub fn retain < F > (& mut self , mut f : F) where F : FnMut (& K , & mut V) -> bool , { let free = self . free ; let mut drop_filtered_values = DropFilteredValues { free : & mut self . free , cur_free : free , } ; self . table . retain (| & mut node | unsafe { let (k , v) = (* node . as_ptr ()) . entry_mut () ; if f (k , v) { true } else { drop_filtered_values . drop_later (node) ; false } }) ; } # [inline] pub fn hasher (& self) -> & S { & self . hash_builder } # [inline] pub fn capacity (& self) -> usize { self . table . capacity () } }
    };
}

impl_4!();