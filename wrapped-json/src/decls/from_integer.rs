macro_rules! deps {
    () => {
        Value!();
        Number!();
    };
}

macro_rules! from_integer {
    () => {
        deps!();
        macro_rules ! from_integer { ($ ($ ty : ident) *) => { $ (impl From <$ ty > for Value { fn from (n : $ ty) -> Self { Value :: Number (n . into ()) } }) * } ; }
    };
}

from_integer!();