macro_rules! deps {
    () => {
        InvariantOpaque!();
    };
}

macro_rules! PassManager {
    () => {
        deps!();
        # [repr (C)] pub (crate) struct PassManager < 'a > (InvariantOpaque < 'a >) ;
    };
}

PassManager!();