macro_rules! deps {
    () => {
        Writable!();
        ContainingDirectory!();
        Handle!();
        AutoRemove!();
    };
}

macro_rules! writable_at {
    () => {
        deps!();
        # [doc = " A shortcut to [`Handle::<Writable>::at()`] providing a writable temporary file at the given path."] pub fn writable_at (path : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove ,) -> io :: Result < Handle < Writable > > { Handle :: < Writable > :: at (path , directory , cleanup) }
    };
}

writable_at!()