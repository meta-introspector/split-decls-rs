macro_rules! deps {
    () => {
        Writing!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Drop for Writing { fn drop (& mut self) { rt :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; assert ! (state . is_writing) ; assert ! (state . is_reading == 0) ; state . is_writing = false ; if ! std :: thread :: panicking () { state . track_write (& execution . threads) ; } }) } }
    };
}

impl_55!();