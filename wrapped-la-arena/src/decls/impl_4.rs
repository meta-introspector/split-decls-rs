macro_rules! deps {
    () => {
        ArenaMap!();
        Idx!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < T , V > Default for ArenaMap < Idx < V > , T > { fn default () -> Self { Self :: new () } }
    };
}

impl_4!();