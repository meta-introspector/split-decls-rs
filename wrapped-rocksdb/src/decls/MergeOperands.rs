macro_rules! MergeOperands {
    () => {
        pub struct MergeOperands { operands_list : * const * const c_char , operands_list_len : * const size_t , num_operands : usize , }
    };
}

MergeOperands!();