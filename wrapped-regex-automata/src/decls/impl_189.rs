macro_rules! deps {
    () => {
        Minimizer!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'a > fmt :: Debug for Minimizer < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Minimizer") . field ("dfa" , & self . dfa) . field ("in_transitions" , & self . in_transitions) . field ("partitions" , & self . partitions) . field ("waiting" , & self . waiting) . finish () } }
    };
}

impl_189!();