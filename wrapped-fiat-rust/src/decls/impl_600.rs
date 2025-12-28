macro_rules! impl_600 {
    () => {
        impl core :: ops :: Index < usize > for fiat_p448_loose_field_element { type Output = u64 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_600!();