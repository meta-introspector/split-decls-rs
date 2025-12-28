macro_rules! AutoderefKind {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum AutoderefKind { # [doc = " A true pointer type, such as `&T` and `*mut T`."] Builtin , # [doc = " A type which must dispatch to a `Deref` implementation."] Overloaded , }
    };
}

AutoderefKind!()