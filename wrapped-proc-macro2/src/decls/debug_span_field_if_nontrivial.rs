macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! debug_span_field_if_nontrivial {
    () => {
        deps!();
        pub (crate) fn debug_span_field_if_nontrivial (debug : & mut fmt :: DebugStruct , span : Span) { # [cfg (span_locations)] { if span . is_call_site () { return ; } } if cfg ! (span_locations) { debug . field ("span" , & span) ; } }
    };
}

debug_span_field_if_nontrivial!()