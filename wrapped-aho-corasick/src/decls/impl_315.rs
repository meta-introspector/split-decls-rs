macro_rules! deps {
    () => {
        ByteClassIter!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl Iterator for ByteClassIter { type Item = u8 ; fn next (& mut self) -> Option < u8 > { self . it . next () . map (| class | class . as_u8 ()) } }
    };
}

impl_315!()