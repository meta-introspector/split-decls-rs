macro_rules! deps {
    () => {
        Shared!();
        Bytes!();
    };
}

macro_rules! shallow_clone_vec {
    () => {
        deps!();
        # [cold] unsafe fn shallow_clone_vec (atom : & AtomicPtr < () > , ptr : * const () , buf : * mut u8 , offset : * const u8 , len : usize ,) -> Bytes { let shared = Box :: new (Shared { buf , cap : offset . offset_from (buf) as usize + len , ref_cnt : AtomicUsize :: new (2) , }) ; let shared = Box :: into_raw (shared) ; debug_assert ! (0 == (shared as usize & KIND_MASK) , "internal: Box<Shared> should have an aligned pointer" ,) ; match atom . compare_exchange (ptr as _ , shared as _ , Ordering :: AcqRel , Ordering :: Acquire) { Ok (actual) => { debug_assert ! (core :: ptr :: eq (actual , ptr)) ; Bytes { ptr : offset , len , data : AtomicPtr :: new (shared as _) , vtable : & SHARED_VTABLE , } } Err (actual) => { let shared = Box :: from_raw (shared) ; mem :: forget (* shared) ; shallow_clone_arc (actual as _ , offset , len) } } }
    };
}

shallow_clone_vec!();