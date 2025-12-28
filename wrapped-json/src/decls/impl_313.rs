macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < T > From < Option < T > > for Value where T : Into < Value > , { fn from (opt : Option < T >) -> Self { match opt { None => Value :: Null , Some (value) => Into :: into (value) , } } }
    };
}

impl_313!()