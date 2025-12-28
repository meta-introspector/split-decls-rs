macro_rules! VariableLengths {
    () => {
        struct VariableLengths { region_constraints_len : usize , type_var_len : usize , int_var_len : usize , float_var_len : usize , const_var_len : usize , }
    };
}

VariableLengths!();