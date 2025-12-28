macro_rules! deps {
    () => {
        CustomSpawn!();
        ThreadSpawn!();
        ThreadBuilder!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < F > ThreadSpawn for CustomSpawn < F > where F : FnMut (ThreadBuilder) -> io :: Result < () > , { private_impl ! { } # [inline] fn spawn (& mut self , thread : ThreadBuilder) -> io :: Result < () > { (self . 0) (thread) } }
    };
}

impl_107!();