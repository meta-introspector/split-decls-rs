macro_rules! StringSink {
    () => {
        # [cfg (any (feature = "alloc" , test))] pub (crate) struct StringSink < 'a > { string : & 'a mut String , }
    };
}

StringSink!();