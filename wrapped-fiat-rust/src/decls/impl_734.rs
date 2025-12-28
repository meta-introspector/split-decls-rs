macro_rules! impl_734 {
    () => {
        impl core :: ops :: IndexMut < usize > for fiat_poly1305_tight_field_element { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . 0 [index] } }
    };
}

impl_734!()