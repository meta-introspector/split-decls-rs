macro_rules! deps {
    () => {
        LazyBuffer!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < I > LazyBuffer < I > where I : Iterator , I :: Item : Clone , { pub fn get_at (& self , indices : & [usize]) -> Vec < I :: Item > { indices . iter () . map (| i | self . buffer [* i] . clone ()) . collect () } pub fn get_array < const K : usize > (& self , indices : [usize ; K]) -> [I :: Item ; K] { indices . map (| i | self . buffer [i] . clone ()) } }
    };
}

impl_341!();