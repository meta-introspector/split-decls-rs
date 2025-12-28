macro_rules! impl_serializable_state_u64_array {
    () => {
        macro_rules ! impl_serializable_state_u64_array { ($ ($ n : ty) ,*) => { $ (impl_serializable_state_type_array ! (u64 , U8 , $ n) ;) * } ; }
    };
}

impl_serializable_state_u64_array!()