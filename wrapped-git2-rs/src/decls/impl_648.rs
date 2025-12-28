macro_rules! deps {
    () => {
        Refspec!();
        Refspecs!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl < 'repo > Iterator for Refspecs < 'repo > { type Item = Refspec < 'repo > ; fn next (& mut self) -> Option < Refspec < 'repo > > { self . range . next () . and_then (| i | self . remote . get_refspec (i)) } fn size_hint (& self) -> (usize , Option < usize >) { self . range . size_hint () } }
    };
}

impl_648!()