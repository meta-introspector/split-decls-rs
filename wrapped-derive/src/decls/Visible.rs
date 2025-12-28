macro_rules! Visible {
    () => {
        # [derive (Debug , Clone)] pub enum Visible { None , HiddenAlways , FnName (Path) , }
    };
}

Visible!();