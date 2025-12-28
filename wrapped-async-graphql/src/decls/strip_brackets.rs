macro_rules! strip_brackets {
    () => {
        fn strip_brackets (type_name : & str) -> Option < & str > { type_name . strip_prefix ('[') . map (| rest | & rest [.. rest . len () - 1]) }
    };
}

strip_brackets!();