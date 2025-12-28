macro_rules! deps {
    () => {
        MapImpl!();
        Value!();
        Map!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [allow (clippy :: derivable_impls)] impl Default for Map < String , Value > { # [inline] fn default () -> Self { Map { map : MapImpl :: new () , } } }
    };
}

impl_80!()