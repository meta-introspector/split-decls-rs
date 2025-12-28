macro_rules! deps {
    () => {
        IntersperseIter!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        impl < I > DoubleEndedIterator for IntersperseIter < I > where I : DoubleEndedIterator < Item : Clone > + ExactSizeIterator , { fn next_back (& mut self) -> Option < Self :: Item > { if self . clone_last { self . clone_last = false ; Some (self . item . clone ()) } else if let next_back @ Some (_) = self . base . next_back () { self . clone_last = self . base . len () != 0 ; next_back } else if self . clone_first { self . clone_first = false ; Some (self . item . clone ()) } else { None } } }
    };
}

impl_652!();