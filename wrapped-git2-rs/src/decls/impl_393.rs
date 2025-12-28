macro_rules! deps {
    () => {
        IndexEntry!();
        IndexEntries!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < 'index > Iterator for IndexEntries < 'index > { type Item = IndexEntry ; fn next (& mut self) -> Option < IndexEntry > { self . range . next () . map (| i | self . index . get (i) . unwrap ()) } }
    };
}

impl_393!()