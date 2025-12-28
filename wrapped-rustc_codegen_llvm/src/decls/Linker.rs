macro_rules! deps {
    () => {
        InvariantOpaque!();
    };
}

macro_rules! Linker {
    () => {
        deps!();
        # [repr (C)] pub (crate) struct Linker < 'a > (InvariantOpaque < 'a >) ;
    };
}

Linker!();