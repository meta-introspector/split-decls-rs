macro_rules! deps {
    () => {
        FileInfo!();
    };
}

macro_rules! SourceMap {
    () => {
        deps!();
        # [cfg (all (span_locations , not (fuzzing)))] struct SourceMap { files : Vec < FileInfo > , }
    };
}

SourceMap!();