macro_rules! deps {
    () => {
        GeneratorObj!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < T , const LOCAL : bool > Iterator for GeneratorObj < '_ , () , T , LOCAL > { type Item = T ; fn next (& mut self) -> Option < T > { self . resume () } }
    };
}

impl_18!()