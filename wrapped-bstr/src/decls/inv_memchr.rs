macro_rules! inv_memchr {
    () => {
        pub fn inv_memchr (n1 : u8 , haystack : & [u8]) -> Option < usize > { let vn1 = repeat_byte (n1) ; let confirm = | byte | byte != n1 ; let loop_size = cmp :: min (LOOP_SIZE , haystack . len ()) ; let start_ptr = haystack . as_ptr () ; unsafe { let end_ptr = haystack . as_ptr () . add (haystack . len ()) ; let mut ptr = start_ptr ; if haystack . len () < USIZE_BYTES { return forward_search (start_ptr , end_ptr , ptr , confirm) ; } let chunk = read_unaligned_usize (ptr) ; if (chunk ^ vn1) != 0 { return forward_search (start_ptr , end_ptr , ptr , confirm) ; } ptr = ptr . add (USIZE_BYTES - (start_ptr as usize & ALIGN_MASK)) ; debug_assert ! (ptr > start_ptr) ; debug_assert ! (end_ptr . sub (USIZE_BYTES) >= start_ptr) ; while loop_size == LOOP_SIZE && ptr <= end_ptr . sub (loop_size) { debug_assert_eq ! (0 , (ptr as usize) % USIZE_BYTES) ; let a = * (ptr as * const usize) ; let b = * (ptr . add (USIZE_BYTES) as * const usize) ; let eqa = (a ^ vn1) != 0 ; let eqb = (b ^ vn1) != 0 ; if eqa || eqb { break ; } ptr = ptr . add (LOOP_SIZE) ; } forward_search (start_ptr , end_ptr , ptr , confirm) } }
    };
}

inv_memchr!()