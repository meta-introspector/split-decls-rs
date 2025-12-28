macro_rules! IntersperseElement {
    () => {
        pub trait IntersperseElement < Item > { fn generate (& mut self) -> Item ; }
    };
}

IntersperseElement!()