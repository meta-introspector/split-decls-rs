macro_rules! deps {
    () => {
        StrippedBytes!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 's > Iterator for StrippedBytes < 's > { type Item = & 's [u8] ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_bytes (& mut self . bytes , & mut self . state , & mut self . utf8parser) } }
    };
}

impl_15!();