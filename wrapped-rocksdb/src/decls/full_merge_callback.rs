macro_rules! deps {
    () => {
        MergeFn!();
        MergeOperatorCallback!();
        MergeOperands!();
    };
}

macro_rules! full_merge_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn full_merge_callback < F : MergeFn , PF : MergeFn > (raw_cb : * mut c_void , raw_key : * const c_char , key_len : size_t , existing_value : * const c_char , existing_value_len : size_t , operands_list : * const * const c_char , operands_list_len : * const size_t , num_operands : c_int , success : * mut u8 , new_value_length : * mut size_t ,) -> * mut c_char { let cb = unsafe { & mut * (raw_cb as * mut MergeOperatorCallback < F , PF >) } ; let operands = & MergeOperands :: new (operands_list , operands_list_len , num_operands) ; let key = unsafe { slice :: from_raw_parts (raw_key as * const u8 , key_len) } ; let oldval = if existing_value . is_null () { None } else { Some (unsafe { slice :: from_raw_parts (existing_value as * const u8 , existing_value_len) }) } ; (cb . full_merge_fn) (key , oldval , operands) . map_or_else (| | { unsafe { * new_value_length = 0 } ; unsafe { * success = 0_u8 } ; ptr :: null_mut () as * mut c_char } , | result | { unsafe { * new_value_length = result . len () as size_t } ; unsafe { * success = 1_u8 } ; Box :: into_raw (result . into_boxed_slice ()) as * mut c_char } ,) }
    };
}

full_merge_callback!()