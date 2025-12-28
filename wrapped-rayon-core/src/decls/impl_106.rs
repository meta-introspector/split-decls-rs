macro_rules! deps {
    () => {
        ThreadBuilder!();
        CustomSpawn!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < F > CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { pub (super) fn new (spawn : F) -> Self { CustomSpawn (spawn) } }
    };
}

impl_106!()