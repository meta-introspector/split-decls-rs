macro_rules! deps {
    () => {
        Engine!();
        EncoderWriter!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'e , E : Engine , W : io :: Write > Drop for EncoderWriter < 'e , E , W > { fn drop (& mut self) { if ! self . panicked { let _ = self . write_final_leftovers () ; } } }
    };
}

impl_47!()