macro_rules! impl_273 {
    () => {
        impl core :: ops :: Index < usize > for fiat_p256_montgomery_domain_field_element { type Output = u64 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_273!();