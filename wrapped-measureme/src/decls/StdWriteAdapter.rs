macro_rules! deps {
    () => {
        SerializationSink!();
    };
}

macro_rules! StdWriteAdapter {
    () => {
        deps!();
        # [doc = " This struct allows to treat `SerializationSink` as `std::io::Write`."] pub struct StdWriteAdapter < 'a > (& 'a SerializationSink) ;
    };
}

StdWriteAdapter!()