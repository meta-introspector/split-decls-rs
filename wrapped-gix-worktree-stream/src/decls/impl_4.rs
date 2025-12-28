macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Source { pub (crate) fn len (& self) -> Option < usize > { match self { Source :: Null => Some (0) , Source :: Path (_) => None , Source :: Memory (buf) => Some (buf . len ()) , } } }
    };
}

impl_4!()