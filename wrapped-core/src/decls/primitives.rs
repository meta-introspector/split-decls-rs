macro_rules! deps {
    () => {
        RuntimeType!();
        ConstBuffer!();
    };
}

macro_rules! primitives {
    () => {
        deps!();
        macro_rules ! primitives { ($ (($ t : ty , $ s : literal)) ,+) => { $ (impl RuntimeType for $ t { const SIGNATURE : imp :: ConstBuffer = imp :: ConstBuffer :: from_slice ($ s) ; }) * } ; }
    };
}

primitives!()