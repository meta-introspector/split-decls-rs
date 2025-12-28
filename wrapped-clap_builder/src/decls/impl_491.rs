macro_rules! deps {
    () => {
        Indices!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl Iterator for Indices < '_ > { type Item = usize ; fn next (& mut self) -> Option < usize > { if let Some (next) = self . iter . next () { self . len -= 1 ; Some (next) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_491!();