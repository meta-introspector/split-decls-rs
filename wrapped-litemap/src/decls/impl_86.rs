macro_rules! deps {
    () => {
        StoreIntoIterator!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < K , V > StoreIntoIterator < K , V > for Vec < (K , V) > { type KeyValueIntoIter = alloc :: vec :: IntoIter < (K , V) > ; # [inline] fn lm_into_iter (self) -> Self :: KeyValueIntoIter { IntoIterator :: into_iter (self) } # [inline] fn lm_extend_end (& mut self , other : Self) { self . extend (other) } # [inline] fn lm_extend_start (& mut self , other : Self) { self . splice (0 .. 0 , other) ; } }
    };
}

impl_86!();