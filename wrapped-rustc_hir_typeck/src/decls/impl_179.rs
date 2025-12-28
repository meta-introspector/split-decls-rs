macro_rules! deps {
    () => {
        ParamTerm!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl ParamTerm { fn index (self) -> usize { match self { ParamTerm :: Ty (ty) => ty . index as usize , ParamTerm :: Const (ct) => ct . index as usize , } } }
    };
}

impl_179!();