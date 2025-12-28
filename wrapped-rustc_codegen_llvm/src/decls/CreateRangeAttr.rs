macro_rules! CreateRangeAttr {
    () => {
        pub (crate) fn CreateRangeAttr (llcx : & Context , size : Size , range : WrappingRange) -> & Attribute { let lower = range . start ; let upper = range . end . wrapping_add (1) ; let as_u64_array = | x : u128 | [x as u64 , (x >> 64) as u64] ; let lower_words : [u64 ; 2] = as_u64_array (lower) ; let upper_words : [u64 ; 2] = as_u64_array (upper) ; let size_bits = size . bits () ; assert ! (size_bits <= 128) ; assert ! (size_bits . div_ceil (64) <= u64 :: try_from (lower_words . len ()) . unwrap ()) ; assert ! (size_bits . div_ceil (64) <= u64 :: try_from (upper_words . len ()) . unwrap ()) ; let size_bits = c_uint :: try_from (size_bits) . unwrap () ; unsafe { LLVMRustCreateRangeAttribute (llcx , size_bits , lower_words . as_ptr () , upper_words . as_ptr ()) } }
    };
}

CreateRangeAttr!()