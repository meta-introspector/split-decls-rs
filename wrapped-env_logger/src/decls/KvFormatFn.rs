macro_rules! deps {
    () => {
        Formatter!();
    };
}

macro_rules! KvFormatFn {
    () => {
        deps!();
        # [doc = " Format function for serializing key/value pairs"] # [doc = ""] # [doc = " This function determines how key/value pairs for structured logs are serialized within the default"] # [doc = " format."] pub (crate) type KvFormatFn = dyn Fn (& mut Formatter , & dyn Source) -> io :: Result < () > + Sync + Send ;
    };
}

KvFormatFn!()