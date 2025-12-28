macro_rules! impl_serializable_state_u32_array {
    () => {
        macro_rules ! impl_serializable_state_u32_array { ($ ($ n : ty) ,*) => { $ (impl_serializable_state_type_array ! (u32 , U4 , $ n) ;) * } ; }
    };
}

impl_serializable_state_u32_array!();