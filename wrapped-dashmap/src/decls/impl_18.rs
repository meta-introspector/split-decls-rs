macro_rules! deps {
    () => {
        OwningIter!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < K : Eq + Hash > OwningIter < K > { pub (crate) fn new (inner : crate :: iter :: OwningIter < K , () >) -> Self { Self { inner } } }
    };
}

impl_18!()