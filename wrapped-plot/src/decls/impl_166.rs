macro_rules! deps {
    () => {
        Plot!();
        Script!();
        Matrix!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl Plot { fn new < S > (data : Matrix , script : & S) -> Plot where S : Script , { Plot { data , script : script . script () , } } fn data (& self) -> & Matrix { & self . data } fn script (& self) -> & str { & self . script } }
    };
}

impl_166!()