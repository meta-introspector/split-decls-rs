macro_rules! deps {
    () => {
        Nullable!();
        NotZero!();
        Zero!();
    };
}

macro_rules! impl_973 {
    () => {
        deps!();
        impl < T : Zero > Nullable for NotZero < T > { # [doc (hidden)] type Wrapped = T ; # [doc (hidden)] fn new (value : T) -> Self { assert ! (! value . is_zero ()) ; NotZero (value) } # [doc (hidden)] fn is_null (& self) -> bool { self . 0 . is_zero () } # [doc (hidden)] fn as_ref (& self) -> Option < & Self :: Wrapped > { if ! self . is_null () { Some (& self . 0) } else { None } } # [doc (hidden)] fn as_mut (& mut self) -> Option < & mut Self :: Wrapped > { if ! self . is_null () { Some (& mut self . 0) } else { None } } }
    };
}

impl_973!()