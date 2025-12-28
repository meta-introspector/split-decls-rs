macro_rules! deps {
    () => {
        Plot!();
        Script!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl Plot { fn new < S > (data : Matrix , script : & S) -> Plot where S : Script , { Plot { data , script : script . script () , } } fn data (& self) -> & Matrix { & self . data } fn script (& self) -> & str { & self . script } }
    };
}

impl_55!()