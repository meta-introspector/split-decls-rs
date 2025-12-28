macro_rules! deps {
    () => {
        Pod!();
    };
}

macro_rules! impl_pod {
    () => {
        deps!();
        macro_rules ! impl_pod { (@ array $ ($ e : expr) ,+) => { $ (unsafe impl < T > Pod for [T ; $ e] where T : Pod { }) + } ; ($ ($ t : ty) +) => { $ (unsafe impl Pod for $ t { }) + } ; }
    };
}

impl_pod!();