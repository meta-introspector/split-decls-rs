macro_rules! deps {
    () => {
        ReflogEntry!();
        ReflogIter!();
    };
}

macro_rules! impl_626 {
    () => {
        deps!();
        impl < 'reflog > Iterator for ReflogIter < 'reflog > { type Item = ReflogEntry < 'reflog > ; fn next (& mut self) -> Option < ReflogEntry < 'reflog > > { self . range . next () . and_then (| i | self . reflog . get (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_626!();