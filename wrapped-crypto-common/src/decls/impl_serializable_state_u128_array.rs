macro_rules! impl_serializable_state_u128_array {
    () => {
        macro_rules ! impl_serializable_state_u128_array { ($ ($ n : ty) ,*) => { $ (impl_serializable_state_type_array ! (u128 , U8 , $ n) ;) * } ; }
    };
}

impl_serializable_state_u128_array!();