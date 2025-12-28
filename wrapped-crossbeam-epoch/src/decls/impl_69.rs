macro_rules! deps {
    () => {
        Collector!();
        Global!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Default for Collector { # [allow (clippy :: arc_with_non_send_sync)] fn default () -> Self { Self { global : Arc :: new (Global :: new ()) , } } }
    };
}

impl_69!()