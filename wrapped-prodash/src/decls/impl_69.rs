macro_rules! deps {
    () => {
        State!();
        Root!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl State { pub (crate) fn update_from_progress (& mut self , progress : & impl Root) -> bool { progress . sorted_snapshot (& mut self . tree) ; let mut hasher = DefaultHasher :: new () ; self . tree . hash (& mut hasher) ; let cur_hash = hasher . finish () ; self . for_next_copy = progress . copy_new_messages (& mut self . messages , self . for_next_copy . take ()) . into () ; let changed = self . tree_hash != cur_hash ; self . tree_hash = cur_hash ; changed } pub (crate) fn clear (& mut self) { self . tree . clear () ; self . messages . clear () ; self . for_next_copy . take () ; } }
    };
}

impl_69!()