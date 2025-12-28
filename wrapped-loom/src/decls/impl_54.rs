macro_rules! deps {
    () => {
        Reading!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl Drop for Reading { fn drop (& mut self) { rt :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; assert ! (state . is_reading > 0) ; assert ! (! state . is_writing) ; state . is_reading -= 1 ; if ! std :: thread :: panicking () { state . track_read (& execution . threads) ; } }) } }
    };
}

impl_54!()