macro_rules! deps {
    () => {
        StripBytesIter!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 's > Iterator for StripBytesIter < 's > { type Item = & 's [u8] ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_bytes (& mut self . bytes , self . state , self . utf8parser) } }
    };
}

impl_19!()