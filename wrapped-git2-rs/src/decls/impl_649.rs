macro_rules! deps {
    () => {
        Refspec!();
        Refspecs!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        impl < 'repo > DoubleEndedIterator for Refspecs < 'repo > { fn next_back (& mut self) -> Option < Refspec < 'repo > > { self . range . next_back () . and_then (| i | self . remote . get_refspec (i)) } }
    };
}

impl_649!()