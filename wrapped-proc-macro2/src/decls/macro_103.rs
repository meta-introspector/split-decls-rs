macro_rules! deps {
    () => {
        SourceMap!();
        FileInfo!();
        Span!();
    };
}

macro_rules! macro_103 {
    () => {
        deps!();
        # [cfg (all (span_locations , not (fuzzing)))] thread_local ! { static SOURCE_MAP : RefCell < SourceMap > = RefCell :: new (SourceMap { files : vec ! [FileInfo { source_text : String :: new () , span : Span { lo : 0 , hi : 0 } , lines : vec ! [0] , char_index_to_byte_offset : BTreeMap :: new () , }] , }) ; }
    };
}

macro_103!()