macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! FileInfo {
    () => {
        deps!();
        # [cfg (all (span_locations , not (fuzzing)))] struct FileInfo { source_text : String , span : Span , lines : Vec < usize > , char_index_to_byte_offset : BTreeMap < usize , usize > , }
    };
}

FileInfo!();