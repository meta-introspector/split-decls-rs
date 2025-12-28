macro_rules! deps {
    () => {
        DroplessArena!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Default for DroplessArena { # [inline] fn default () -> DroplessArena { DroplessArena { start : Cell :: new (ptr :: null_mut ()) , end : Cell :: new (ptr :: null_mut ()) , chunks : Default :: default () , } } }
    };
}

impl_16!()