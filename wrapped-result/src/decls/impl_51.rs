macro_rules! deps {
    () => {
        HeapString!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Drop for HeapString { fn drop (& mut self) { if ! self . 0 . is_null () { unsafe { HeapFree (GetProcessHeap () , 0 , self . 0 as _) ; } } } }
    };
}

impl_51!()