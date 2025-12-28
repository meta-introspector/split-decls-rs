macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [doc (hidden)] impl crate :: Level { pub const fn into_tracing_level (self) -> tracing_core :: Level { match self { crate :: Level :: Coarse => tracing_core :: Level :: INFO , crate :: Level :: Detail => tracing_core :: Level :: DEBUG , } } }
    };
}

impl_8!()