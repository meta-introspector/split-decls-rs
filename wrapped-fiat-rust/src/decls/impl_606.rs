macro_rules! impl_606 {
    () => {
        impl core :: ops :: IndexMut < usize > for fiat_p448_tight_field_element { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . 0 [index] } }
    };
}

impl_606!();