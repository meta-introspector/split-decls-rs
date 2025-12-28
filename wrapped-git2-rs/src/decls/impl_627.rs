macro_rules! deps {
    () => {
        ReflogEntry!();
        ReflogIter!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        impl < 'reflog > DoubleEndedIterator for ReflogIter < 'reflog > { fn next_back (& mut self) -> Option < ReflogEntry < 'reflog > > { self . range . next_back () . and_then (| i | self . reflog . get (i)) } }
    };
}

impl_627!()