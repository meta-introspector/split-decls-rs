macro_rules! deps {
    () => {
        EdgeType!();
        Generator!();
        Graph!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        impl < Ty : EdgeType > Iterator for Generator < Ty > { type Item = Graph < () , () , Ty > ; fn next (& mut self) -> Option < Self :: Item > { self . next_ref () . cloned () } }
    };
}

impl_595!()