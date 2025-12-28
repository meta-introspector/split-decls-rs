macro_rules! EncodingVisitor {
    () => {
        # [cfg (feature = "serde")] struct EncodingVisitor ;
    };
}

EncodingVisitor!();