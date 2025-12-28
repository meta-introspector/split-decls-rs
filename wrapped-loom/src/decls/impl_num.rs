macro_rules! deps {
    () => {
        Numeric!();
    };
}

macro_rules! impl_num {
    () => {
        deps!();
        macro_rules ! impl_num { ($ ($ t : ty) ,*) => { $ (impl Numeric for $ t { fn into_u64 (self) -> u64 { self as u64 } fn from_u64 (src : u64) -> $ t { src as $ t } }) * } ; }
    };
}

impl_num!()