macro_rules! impl_569 {
    () => {
        impl core :: ops :: Index < usize > for fiat_p448_loose_field_element { type Output = u32 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_569!();