macro_rules! impl_80 {
    () => {
        impl core :: ops :: IndexMut < usize > for fiat_25519_scalar_non_montgomery_domain_field_element { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . 0 [index] } }
    };
}

impl_80!()