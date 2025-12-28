macro_rules! deps {
    () => {
        Plot!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl Plot { fn new (name : & str , url : & str) -> Plot { Plot { name : name . to_owned () , url : url . to_owned () , } } }
    };
}

impl_135!();