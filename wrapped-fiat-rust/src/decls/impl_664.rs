macro_rules! impl_664 {
    () => {
        impl core :: ops :: Index < usize > for fiat_p521_loose_field_element { type Output = u64 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_664!()