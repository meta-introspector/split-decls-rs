macro_rules! deps {
    () => {
        AccelTy!();
        IterAccels!();
        Accel!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < 'a , A : AsRef < [AccelTy] > > Iterator for IterAccels < 'a , A > { type Item = Accel ; fn next (& mut self) -> Option < Accel > { let accel = self . accels . get (self . i) ? ; self . i += 1 ; Some (accel) } }
    };
}

impl_164!();