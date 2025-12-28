macro_rules! deps {
    () => {
        Matrix3x2!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl windows_core :: RuntimeType for Matrix3x2 { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: from_slice (b"struct(Windows.Foundation.Numerics.Matrix3x2;f4;f4;f4;f4;f4;f4)" ,) ; }
    };
}

impl_2!()