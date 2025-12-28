macro_rules! deps {
    () => {
        StoreMut!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < K , V > StoreMut < K , V > for Vec < (K , V) > { # [inline] fn lm_with_capacity (capacity : usize) -> Self { Self :: with_capacity (capacity) } # [inline] fn lm_reserve (& mut self , additional : usize) { self . reserve (additional) } # [inline] fn lm_get_mut (& mut self , index : usize) -> Option < (& K , & mut V) > { self . as_mut_slice () . get_mut (index) . map (map_f_mut) } # [inline] fn lm_push (& mut self , key : K , value : V) { self . push ((key , value)) } # [inline] fn lm_insert (& mut self , index : usize , key : K , value : V) { self . insert (index , (key , value)) } # [inline] fn lm_remove (& mut self , index : usize) -> (K , V) { self . remove (index) } # [inline] fn lm_clear (& mut self) { self . clear () } }
    };
}

impl_80!()