macro_rules! deps {
    () => {
        WinconBytesIter!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Iterator for WinconBytesIter < '_ > { type Item = (anstyle :: Style , String) ; # [inline] fn next (& mut self) -> Option < Self :: Item > { next_bytes (& mut self . bytes , self . parser , self . capture) } }
    };
}

impl_31!()