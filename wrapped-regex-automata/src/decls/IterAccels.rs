macro_rules! deps {
    () => {
        AccelTy!();
        Accels!();
    };
}

macro_rules! IterAccels {
    () => {
        deps!();
        # [derive (Debug)] struct IterAccels < 'a , A : AsRef < [AccelTy] > > { accels : & 'a Accels < A > , i : usize , }
    };
}

IterAccels!();