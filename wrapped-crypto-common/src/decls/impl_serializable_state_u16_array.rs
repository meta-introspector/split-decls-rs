macro_rules! impl_serializable_state_u16_array {
    () => {
        macro_rules ! impl_serializable_state_u16_array { ($ ($ n : ty) ,*) => { $ (impl_serializable_state_type_array ! (u16 , U2 , $ n) ;) * } ; }
    };
}

impl_serializable_state_u16_array!();