macro_rules! deps {
    () => {
        SerializationSink!();
        StringId!();
        Addr!();
    };
}

macro_rules! serialize_index_entry {
    () => {
        deps!();
        fn serialize_index_entry (sink : & SerializationSink , id : StringId , addr : Addr) { sink . write_atomic (16 , | bytes | { bytes [0 .. 8] . copy_from_slice (& id . 0 . to_le_bytes ()) ; bytes [8 .. 16] . copy_from_slice (& addr . 0 . to_le_bytes ()) ; }) ; }
    };
}

serialize_index_entry!();