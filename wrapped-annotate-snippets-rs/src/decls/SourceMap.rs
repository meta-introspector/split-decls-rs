macro_rules! deps {
    () => {
        LineInfo!();
    };
}

macro_rules! SourceMap {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct SourceMap < 'a > { lines : Vec < LineInfo < 'a > > , pub (crate) source : & 'a str , }
    };
}

SourceMap!();