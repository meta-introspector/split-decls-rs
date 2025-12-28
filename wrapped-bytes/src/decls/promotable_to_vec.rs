macro_rules! promotable_to_vec {
    () => {
        unsafe fn promotable_to_vec (data : & AtomicPtr < () > , ptr : * const u8 , len : usize , f : fn (* mut ()) -> * mut u8 ,) -> Vec < u8 > { let shared = data . load (Ordering :: Acquire) ; let kind = shared as usize & KIND_MASK ; if kind == KIND_ARC { shared_to_vec_impl (shared . cast () , ptr , len) } else { debug_assert_eq ! (kind , KIND_VEC) ; let buf = f (shared) ; let cap = ptr . offset_from (buf) as usize + len ; ptr :: copy (ptr , buf , len) ; Vec :: from_raw_parts (buf , len , cap) } }
    };
}

promotable_to_vec!()