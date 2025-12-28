macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_data {
    () => {
        deps!();
        macro_rules ! impl_data { ($ ($ ty : ty) ,+) => { $ (impl Data for $ ty { fn f64 (self) -> f64 { f64 :: cast (self) } } impl <'a > Data for &'a $ ty { fn f64 (self) -> f64 { f64 :: cast (* self) } }) + } }
    };
}

impl_data!()