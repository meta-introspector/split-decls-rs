macro_rules! deps {
    () => {
        SpanLowerer!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl SpanLowerer { fn lower (& self , span : Span) -> Span { if self . is_incremental { span . with_parent (Some (self . def_id)) } else { span } } }
    };
}

impl_100!()