macro_rules! Slice {
    () => {
        pub trait Slice < 's > : AsRef < str > + Clone + PartialEq { fn slice (& self , range : Range < usize >) -> Self ; fn trim (& mut self) ; }
    };
}

Slice!()