macro_rules! deps {
    () => {
        ExpansionSpanMap!();
    };
}

macro_rules! SpanMap {
    () => {
        deps!();
        # [doc = " Spanmap for a macro file or a real file"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum SpanMap { # [doc = " Spanmap for a macro file"] ExpansionSpanMap (Arc < ExpansionSpanMap >) , # [doc = " Spanmap for a real file"] RealSpanMap (Arc < RealSpanMap >) , }
    };
}

SpanMap!()