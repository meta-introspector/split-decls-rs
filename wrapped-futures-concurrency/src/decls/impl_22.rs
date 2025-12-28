macro_rules! deps {
    () => {
        OutputArray!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < T , const N : usize > OutputArray < T , N > { # [doc = " Initialize a new array as uninitialized"] pub (crate) fn uninit () -> Self { Self { data : array :: from_fn (| _ | MaybeUninit :: uninit ()) , } } # [doc = " Write a value into memory at the index"] pub (crate) fn write (& mut self , idx : usize , value : T) { self . data [idx] = MaybeUninit :: new (value) ; } # [doc = " Drop a value at the index"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The value at the index must be initialized"] pub (crate) unsafe fn drop (& mut self , idx : usize) { unsafe { self . data [idx] . assume_init_drop () } ; } # [doc = " Assume all items are initialized and take the items,"] # [doc = " leaving behind uninitialized data."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Make sure that all items are initialized prior to calling this method."] pub (crate) unsafe fn take (& mut self) -> [T ; N] { let mut data = array :: from_fn (| _ | MaybeUninit :: uninit ()) ; mem :: swap (& mut self . data , & mut data) ; unsafe { utils :: array_assume_init (data) } } }
    };
}

impl_22!();