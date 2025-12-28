macro_rules! deps {
    () => {
        Constness!();
        FnKind!();
        FnHeader!();
        Closure!();
        IsAsync!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl < 'a > FnKind < 'a > { pub fn header (& self) -> Option < & FnHeader > { match * self { FnKind :: ItemFn (_ , _ , ref header) => Some (header) , FnKind :: Method (_ , ref sig) => Some (& sig . header) , FnKind :: Closure => None , } } pub fn constness (self) -> Constness { self . header () . map_or (Constness :: NotConst , | header | header . constness) } pub fn asyncness (self) -> IsAsync { self . header () . map_or (IsAsync :: NotAsync , | header | header . asyncness) } }
    };
}

impl_348!();