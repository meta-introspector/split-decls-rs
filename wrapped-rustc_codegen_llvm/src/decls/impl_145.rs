macro_rules! deps {
    () => {
        ValueIter!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'll > Iterator for ValueIter < 'll > { type Item = & 'll Value ; fn next (& mut self) -> Option < & 'll Value > { let old = self . cur ; if let Some (old) = old { self . cur = unsafe { (self . step) (old) } ; } old } }
    };
}

impl_145!()