macro_rules! deps {
    () => {
        Documentation!();
    };
}

macro_rules! DocsRangeMap {
    () => {
        deps!();
        # [doc = " A struct to map text ranges from [`Documentation`] back to TextRanges in the syntax tree."] # [derive (Debug)] pub struct DocsRangeMap { source_map : AttrSourceMap , mapping : Vec < (TextRange , AttrId , TextRange) > , }
    };
}

DocsRangeMap!()