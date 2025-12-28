macro_rules! deps {
    () => {
        ContainingDirectory!();
        Handle!();
        AutoRemove!();
    };
}

macro_rules! new {
    () => {
        deps!();
        # [doc = " A shortcut to [`Handle::<Writable>::new()`], creating a writable temporary file with non-clashing name in a directory."] pub fn new (containing_directory : impl AsRef < Path > , directory : ContainingDirectory , cleanup : AutoRemove ,) -> io :: Result < Handle < Writable > > { Handle :: < Writable > :: new (containing_directory , directory , cleanup) }
    };
}

new!()