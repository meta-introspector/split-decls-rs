macro_rules! impl_111 {
    () => {
        impl core :: ops :: Index < usize > for fiat_25519_scalar_montgomery_domain_field_element { type Output = u64 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_111!()