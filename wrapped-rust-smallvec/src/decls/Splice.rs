macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! Splice {
    () => {
        deps!();
        pub struct Splice < 'a , I : Iterator + 'a , const N : usize > { drain : Drain < 'a , I :: Item , N > , replace_with : I , }
    };
}

Splice!()