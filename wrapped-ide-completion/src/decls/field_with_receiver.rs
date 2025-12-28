macro_rules! field_with_receiver {
    () => {
        fn field_with_receiver (receiver : Option < & str > , field_name : & str) -> SmolStr { receiver . map_or_else (| | field_name . into () , | receiver | format_smolstr ! ("{}.{field_name}" , receiver)) }
    };
}

field_with_receiver!()