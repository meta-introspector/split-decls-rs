macro_rules! deps {
    () => {
        Idx!();
        IntoIter!();
        RawIdx!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T > Iterator for IntoIter < T > { type Item = (Idx < T > , T) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| (idx , value) | (Idx :: from_raw (RawIdx (idx as u32)) , value)) } }
    };
}

impl_56!();