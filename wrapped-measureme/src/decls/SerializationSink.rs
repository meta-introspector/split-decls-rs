macro_rules! deps {
    () => {
        PageTag!();
        SerializationSinkInner!();
        SharedState!();
    };
}

macro_rules! SerializationSink {
    () => {
        deps!();
        # [derive (Debug)] pub struct SerializationSink { shared_state : SharedState , data : Mutex < SerializationSinkInner > , page_tag : PageTag , }
    };
}

SerializationSink!()