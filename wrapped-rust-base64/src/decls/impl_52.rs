macro_rules! deps {
    () => {
        EncoderStringWriter!();
        StrConsumer!();
        Engine!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'e , E : Engine , S : StrConsumer > io :: Write for EncoderStringWriter < 'e , E , S > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . encoder . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . encoder . flush () } }
    };
}

impl_52!()