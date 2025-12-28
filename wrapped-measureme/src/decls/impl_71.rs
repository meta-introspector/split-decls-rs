macro_rules! deps {
    () => {
        SerializationSink!();
        SerializationSinkInner!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Drop for SerializationSink { fn drop (& mut self) { let mut data = self . data . lock () ; let SerializationSinkInner { ref mut buffer , addr : _ , } = * data ; self . flush (buffer) ; } }
    };
}

impl_71!()