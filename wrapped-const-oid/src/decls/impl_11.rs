macro_rules! deps {
    () => {
        Arcs!();
        Arc!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Iterator for Arcs < '_ > { type Item = Arc ; fn next (& mut self) -> Option < Arc > { self . try_next () . expect ("OID malformed") } }
    };
}

impl_11!();