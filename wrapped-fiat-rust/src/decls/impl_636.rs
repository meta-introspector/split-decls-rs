macro_rules! impl_636 {
    () => {
        impl core :: ops :: Index < usize > for fiat_p521_tight_field_element { type Output = u32 ; # [inline] fn index (& self , index : usize) -> & Self :: Output { & self . 0 [index] } }
    };
}

impl_636!();