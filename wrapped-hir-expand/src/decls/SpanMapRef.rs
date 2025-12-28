macro_rules! deps {
    () => {
        ExpansionSpanMap!();
    };
}

macro_rules! SpanMapRef {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub enum SpanMapRef < 'a > { # [doc = " Spanmap for a macro file"] ExpansionSpanMap (& 'a ExpansionSpanMap) , # [doc = " Spanmap for a real file"] RealSpanMap (& 'a RealSpanMap) , }
    };
}

SpanMapRef!()