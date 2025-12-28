macro_rules! deps {
    () => {
        IndexIter!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl Iterator for IndexIter { type Item = usize ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| pos | (pos + self . offset) . wrapping_rem (self . iter . end)) } }
    };
}

impl_19!();