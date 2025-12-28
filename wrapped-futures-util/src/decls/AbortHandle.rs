macro_rules! deps {
    () => {
        AbortInner!();
    };
}

macro_rules! AbortHandle {
    () => {
        deps!();
        # [doc = " A handle to an `Abortable` task."] # [derive (Debug , Clone)] pub struct AbortHandle { inner : Arc < AbortInner > , }
    };
}

AbortHandle!();