macro_rules! impl_353 {
    () => {
        impl core :: ops :: IndexMut < usize > for fiat_p256_scalar_non_montgomery_domain_field_element { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . 0 [index] } }
    };
}

impl_353!()