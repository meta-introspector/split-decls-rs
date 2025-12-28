macro_rules! deps {
    () => {
        Literal!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl Literal { pub fn negate (self) -> Option < Self > { if let Literal :: Int (i , k) = self { Some (Literal :: Int (- i , k)) } else { None } } }
    };
}

impl_224!()