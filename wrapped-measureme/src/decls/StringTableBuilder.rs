macro_rules! deps {
    () => {
        SerializationSink!();
    };
}

macro_rules! StringTableBuilder {
    () => {
        deps!();
        # [doc = " Write-only version of the string table"] pub struct StringTableBuilder { data_sink : Arc < SerializationSink > , index_sink : Arc < SerializationSink > , }
    };
}

StringTableBuilder!()