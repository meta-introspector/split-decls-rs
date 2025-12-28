macro_rules! impl_167 {
    () => {
        impl core :: ops :: Index < usize > for fiat_p224_non_montgomery_domain_field_element { type Output = u32 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_167!();