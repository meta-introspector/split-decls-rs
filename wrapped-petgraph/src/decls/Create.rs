macro_rules! deps {
    () => {
        Build!();
    };
}

macro_rules! Create {
    () => {
        deps!();
        # [doc = " A graph that can be created"] pub trait Create : Build + Default { fn with_capacity (nodes : usize , edges : usize) -> Self ; }
    };
}

Create!();