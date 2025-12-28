macro_rules! deps {
    () => {
        Nullable!();
    };
}

macro_rules! impl_970 {
    () => {
        deps!();
        impl < T > Nullable for Option < T > { type Wrapped = T ; fn new (value : T) -> Self { Some (value) } fn as_ref (& self) -> Option < & Self :: Wrapped > { self . as_ref () } fn as_mut (& mut self) -> Option < & mut Self :: Wrapped > { self . as_mut () } }
    };
}

impl_970!()