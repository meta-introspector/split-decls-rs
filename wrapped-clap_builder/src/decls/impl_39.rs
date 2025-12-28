macro_rules! deps {
    () => {
        AppFlags!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl std :: ops :: BitOr for AppFlags { type Output = Self ; fn bitor (mut self , rhs : Self) -> Self :: Output { self . insert (rhs) ; self } }
    };
}

impl_39!()