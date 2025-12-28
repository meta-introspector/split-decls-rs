macro_rules! deps {
    () => {
        ExpandDatabase!();
        HirFileId!();
        SpanMap!();
        ExpansionSpanMap!();
        SpanMapRef!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl SpanMap { pub fn span_for_range (& self , range : TextRange) -> Span { match self { Self :: ExpansionSpanMap (span_map) => span_map . span_at (range . start ()) , Self :: RealSpanMap (span_map) => span_map . span_for_range (range) , } } pub fn as_ref (& self) -> SpanMapRef < '_ > { match self { Self :: ExpansionSpanMap (span_map) => SpanMapRef :: ExpansionSpanMap (span_map) , Self :: RealSpanMap (span_map) => SpanMapRef :: RealSpanMap (span_map) , } } # [inline] pub (crate) fn new (db : & dyn ExpandDatabase , file_id : HirFileId) -> SpanMap { match file_id { HirFileId :: FileId (file_id) => SpanMap :: RealSpanMap (db . real_span_map (file_id)) , HirFileId :: MacroFile (m) => { SpanMap :: ExpansionSpanMap (db . parse_macro_expansion (m) . value . 1) } } } }
    };
}

impl_183!()