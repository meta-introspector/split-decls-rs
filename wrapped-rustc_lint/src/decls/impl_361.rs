macro_rules! deps {
    () => {
        LifetimeSyntaxCategories!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl std :: ops :: Add for LifetimeSyntaxCategories < usize > { type Output = Self ; fn add (self , rhs : Self) -> Self :: Output { Self { hidden : self . hidden + rhs . hidden , elided : self . elided + rhs . elided , named : self . named + rhs . named , } } }
    };
}

impl_361!()