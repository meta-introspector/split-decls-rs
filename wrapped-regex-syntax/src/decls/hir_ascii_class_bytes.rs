macro_rules! deps {
    () => {
        ClassBytesRange!();
        ClassAsciiKind!();
        ClassBytes!();
    };
}

macro_rules! hir_ascii_class_bytes {
    () => {
        deps!();
        fn hir_ascii_class_bytes (kind : & ast :: ClassAsciiKind) -> hir :: ClassBytes { let ranges : Vec < _ > = ascii_class (kind) . map (| (s , e) | hir :: ClassBytesRange :: new (s , e)) . collect () ; hir :: ClassBytes :: new (ranges) }
    };
}

hir_ascii_class_bytes!();