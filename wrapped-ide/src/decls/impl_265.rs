macro_rules! deps {
    () => {
        LazyProperty!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl < T > LazyProperty < T > { pub fn computed (self) -> Option < T > { match self { LazyProperty :: Computed (it) => Some (it) , _ => None , } } pub fn is_lazy (& self) -> bool { matches ! (self , Self :: Lazy) } }
    };
}

impl_265!()