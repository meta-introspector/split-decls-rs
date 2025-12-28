macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'i , K : Clone + Hash + Eq , V : Clone > Clone for Iter < 'i , K , V > { fn clone (& self) -> Self { Iter { shards : self . shards . clone () , current : self . current . clone () , } } }
    };
}

impl_9!()