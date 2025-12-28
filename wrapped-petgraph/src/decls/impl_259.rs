macro_rules! deps {
    () => {
        Acyclic!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl < G : Default + Visitable > Default for Acyclic < G > { fn default () -> Self { let graph : G = Default :: default () ; let order_map = Default :: default () ; let discovered = RefCell :: new (FixedBitSet :: default ()) ; let finished = RefCell :: new (FixedBitSet :: default ()) ; Self { graph , order_map , discovered , finished , } } }
    };
}

impl_259!()