macro_rules! impl_637 {
    () => {
        impl core :: ops :: IndexMut < usize > for fiat_p521_tight_field_element { # [inline] fn index_mut (& mut self , index : usize) -> & mut Self :: Output { & mut self . 0 [index] } }
    };
}

impl_637!()