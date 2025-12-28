macro_rules! impl_48 {
    () => {
        impl core :: ops :: IndexMut < usize > for fiat_25519_tight_field_element { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . 0 [index] } }
    };
}

impl_48!();