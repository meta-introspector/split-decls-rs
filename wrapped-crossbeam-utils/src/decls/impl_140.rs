macro_rules! deps {
    () => {
        Registration!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl Drop for Registration { fn drop (& mut self) { let mut indices = thread_indices () . lock () . unwrap () ; indices . mapping . remove (& self . thread_id) ; indices . free_list . push (self . index) ; } }
    };
}

impl_140!()