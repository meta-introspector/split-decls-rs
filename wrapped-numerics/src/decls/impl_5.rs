macro_rules! deps {
    () => {
        Matrix4x4!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl windows_core :: RuntimeType for Matrix4x4 { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: from_slice (b"struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4)") ; }
    };
}

impl_5!()