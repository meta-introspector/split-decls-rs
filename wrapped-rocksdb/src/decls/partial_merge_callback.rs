macro_rules! deps {
    () => {
        MergeFn!();
        MergeOperatorCallback!();
        MergeOperands!();
    };
}

macro_rules! partial_merge_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn partial_merge_callback < F : MergeFn , PF : MergeFn > (raw_cb : * mut c_void , raw_key : * const c_char , key_len : size_t , operands_list : * const * const c_char , operands_list_len : * const size_t , num_operands : c_int , success : * mut u8 , new_value_length : * mut size_t ,) -> * mut c_char { let cb = unsafe { & mut * (raw_cb as * mut MergeOperatorCallback < F , PF >) } ; let operands = & MergeOperands :: new (operands_list , operands_list_len , num_operands) ; let key = unsafe { slice :: from_raw_parts (raw_key as * const u8 , key_len) } ; (cb . partial_merge_fn) (key , None , operands) . map_or_else (| | { unsafe { * new_value_length = 0 } ; unsafe { * success = 0_u8 } ; ptr :: null_mut :: < c_char > () } , | result | { unsafe { * new_value_length = result . len () as size_t } ; unsafe { * success = 1_u8 } ; Box :: into_raw (result . into_boxed_slice ()) as * mut c_char } ,) }
    };
}

partial_merge_callback!();