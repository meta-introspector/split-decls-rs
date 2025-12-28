macro_rules! deps {
    () => {
        StaticLifetime!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl StaticLifetime { pub fn name (self) -> Name { Name :: new_symbol_root (sym :: tick_static) } }
    };
}

impl_97!()