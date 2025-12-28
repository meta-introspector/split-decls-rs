macro_rules! Description {
    () => {
        # [doc (hidden)] pub trait Description { fn description () -> & 'static str ; }
    };
}

Description!();