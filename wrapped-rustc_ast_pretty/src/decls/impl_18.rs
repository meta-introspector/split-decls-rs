macro_rules! deps {
    () => {
        BoxMarker!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Drop for BoxMarker { fn drop (& mut self) { panic ! ("BoxMarker not ended with `Printer::end()`") ; } }
    };
}

impl_18!();