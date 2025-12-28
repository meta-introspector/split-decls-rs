macro_rules! deps {
    () => {
        ArgFlags!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl std :: ops :: BitOr for ArgFlags { type Output = Self ; fn bitor (mut self , rhs : Self) -> Self :: Output { self . insert (rhs) ; self } }
    };
}

impl_71!()