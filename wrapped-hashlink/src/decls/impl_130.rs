macro_rules! deps {
    () => {
        DropFilteredValues!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < K , V > Drop for DropFilteredValues < '_ , K , V > { fn drop (& mut self) { unsafe { let end_free = self . cur_free ; while self . cur_free != * self . free { let cur_free = self . cur_free . as_ptr () ; (* cur_free) . take_entry () ; self . cur_free = (* cur_free) . links . free . next ; } * self . free = end_free ; } } }
    };
}

impl_130!();