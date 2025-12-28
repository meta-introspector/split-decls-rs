macro_rules! deps {
    () => {
        Vector3!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl windows_core :: RuntimeType for Vector3 { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: from_slice (b"struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4)" ,) ; }
    };
}

impl_11!();