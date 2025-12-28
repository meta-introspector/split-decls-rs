macro_rules! deps {
    () => {
        OutputVec!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < T > OutputVec < T > { # [doc = " Initialize a new vector as uninitialized"] pub (crate) fn uninit (capacity : usize) -> Self { Self { data : Vec :: with_capacity (capacity) , capacity , } } # [doc = " Write a value into memory at the index"] pub (crate) fn write (& mut self , idx : usize , value : T) { let data = self . data . spare_capacity_mut () ; data [idx] = MaybeUninit :: new (value) ; } # [doc = " Drop a value at the index"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The value at the index must be initialized"] pub (crate) unsafe fn drop (& mut self , idx : usize) { let data = self . data . spare_capacity_mut () ; unsafe { data [idx] . assume_init_drop () } ; } # [doc = " Assume all items are initialized and take the items,"] # [doc = " leaving behind an empty vector"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Make sure that all items are initialized prior to calling this method."] pub (crate) unsafe fn take (& mut self) -> Vec < T > { let mut data = vec ! [] ; mem :: swap (& mut self . data , & mut data) ; unsafe { data . set_len (self . capacity) } ; data } }
    };
}

impl_25!();