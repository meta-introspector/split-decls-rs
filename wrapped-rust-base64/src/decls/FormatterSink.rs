macro_rules! FormatterSink {
    () => {
        struct FormatterSink < 'a , 'b : 'a > { f : & 'a mut Formatter < 'b > , }
    };
}

FormatterSink!()