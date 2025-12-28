macro_rules! impl_729 {
    () => {
        impl core :: ops :: IndexMut < usize > for fiat_poly1305_loose_field_element { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . 0 [index] } }
    };
}

impl_729!();