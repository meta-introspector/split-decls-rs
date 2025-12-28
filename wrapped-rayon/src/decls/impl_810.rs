macro_rules! deps {
    () => {
        RepeatNProducer!();
    };
}

macro_rules! impl_810 {
    () => {
        deps!();
        impl < T : Clone > DoubleEndedIterator for RepeatNProducer < T > { # [inline] fn next_back (& mut self) -> Option < T > { self . next () } # [inline] fn nth_back (& mut self , n : usize) -> Option < T > { self . nth (n) } }
    };
}

impl_810!();