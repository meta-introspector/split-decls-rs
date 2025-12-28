macro_rules! deps {
    () => {
        LifetimeSyntaxCategories!();
        LifetimeSyntaxCategory!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < T > LifetimeSyntaxCategories < T > { fn select (& mut self , category : LifetimeSyntaxCategory) -> & mut T { use LifetimeSyntaxCategory :: * ; match category { Elided => & mut self . elided , Hidden => & mut self . hidden , Named => & mut self . named , } } }
    };
}

impl_359!()