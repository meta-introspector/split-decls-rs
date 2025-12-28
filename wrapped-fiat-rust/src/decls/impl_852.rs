macro_rules! impl_852 {
    () => {
        impl core :: ops :: Index < usize > for fiat_secp256k1_montgomery_scalar_non_montgomery_domain_field_element { type Output = u32 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_852!()