macro_rules! deps {
    () => {
        Closed!();
        AutoRemove!();
        Handle!();
        ContainingDirectory!();
    };
}

macro_rules! mark_at {
    () => {
        deps!();
        # [doc = " A shortcut to [`Handle::<Closed>::at()`] providing a closed temporary file to mark the presence of something."] pub fn mark_at (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove ,) -> io :: Result < Handle < Closed > > { Handle :: < Closed > :: at (path , directory , cleanup) }
    };
}

mark_at!();