macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Config { pub fn new () -> Self { Config { bins : None , generated_output_dir : None , } } }
    };
}

impl_48!();