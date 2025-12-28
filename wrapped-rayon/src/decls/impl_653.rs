macro_rules! deps {
    () => {
        IntersperseIter!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        impl < I > ExactSizeIterator for IntersperseIter < I > where I : DoubleEndedIterator < Item : Clone > + ExactSizeIterator , { fn len (& self) -> usize { let len = self . base . len () ; len + len . saturating_sub (1) + self . clone_first as usize + self . clone_last as usize } }
    };
}

impl_653!();