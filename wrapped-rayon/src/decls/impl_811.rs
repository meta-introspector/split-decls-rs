macro_rules! deps {
    () => {
        Empty!();
        RepeatNProducer!();
    };
}

macro_rules! impl_811 {
    () => {
        deps!();
        impl < T : Clone > ExactSizeIterator for RepeatNProducer < T > { # [inline] fn len (& self) -> usize { match self { Self :: Repeats (_ , count) => count . get () , Self :: Empty => 0 , } } }
    };
}

impl_811!()