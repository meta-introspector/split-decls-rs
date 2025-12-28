macro_rules! deps {
    () => {
        MacroCallId!();
        ExpansionSpanMap!();
        ExpandDatabase!();
    };
}

macro_rules! expansion_span_map {
    () => {
        deps!();
        pub (crate) fn expansion_span_map (db : & dyn ExpandDatabase , file_id : MacroCallId ,) -> Arc < ExpansionSpanMap > { db . parse_macro_expansion (file_id) . value . 1 }
    };
}

expansion_span_map!()