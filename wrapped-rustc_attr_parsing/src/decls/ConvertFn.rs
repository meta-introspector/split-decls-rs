macro_rules! ConvertFn {
    () => {
        type ConvertFn < E > = fn (ThinVec < E > , Span) -> AttributeKind ;
    };
}

ConvertFn!();