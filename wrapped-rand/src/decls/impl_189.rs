macro_rules! deps {
    () => {
        SampleUniform!();
        WeightedIndexIter!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < X > Iterator for WeightedIndexIter < '_ , X > where X : for < 'b > core :: ops :: SubAssign < & 'b X > + SampleUniform + PartialOrd + Clone , { type Item = X ; fn next (& mut self) -> Option < Self :: Item > { match self . weighted_index . weight (self . index) { None => None , Some (weight) => { self . index += 1 ; Some (weight) } } } }
    };
}

impl_189!()