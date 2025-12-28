macro_rules! deps {
    () => {
        UninitSlice!();
    };
}

macro_rules! impl_index {
    () => {
        deps!();
        macro_rules ! impl_index { ($ ($ t : ty) ,*) => { $ (impl Index <$ t > for UninitSlice { type Output = UninitSlice ; # [inline] fn index (& self , index : $ t) -> & UninitSlice { UninitSlice :: uninit_ref (& self . 0 [index]) } } impl IndexMut <$ t > for UninitSlice { # [inline] fn index_mut (& mut self , index : $ t) -> & mut UninitSlice { UninitSlice :: uninit (& mut self . 0 [index]) } }) * } ; }
    };
}

impl_index!()