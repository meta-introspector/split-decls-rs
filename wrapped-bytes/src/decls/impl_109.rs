macro_rules! deps {
    () => {
        Bytes!();
        Shared!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl From < Vec < u8 > > for Bytes { fn from (vec : Vec < u8 >) -> Bytes { let mut vec = ManuallyDrop :: new (vec) ; let ptr = vec . as_mut_ptr () ; let len = vec . len () ; let cap = vec . capacity () ; if len == cap { let vec = ManuallyDrop :: into_inner (vec) ; return Bytes :: from (vec . into_boxed_slice ()) ; } let shared = Box :: new (Shared { buf : ptr , cap , ref_cnt : AtomicUsize :: new (1) , }) ; let shared = Box :: into_raw (shared) ; debug_assert ! (0 == (shared as usize & KIND_MASK) , "internal: Box<Shared> should have an aligned pointer" ,) ; Bytes { ptr , len , data : AtomicPtr :: new (shared as _) , vtable : & SHARED_VTABLE , } } }
    };
}

impl_109!();