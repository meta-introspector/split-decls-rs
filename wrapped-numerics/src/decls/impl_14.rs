macro_rules! deps {
    () => {
        Vector4!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl windows_core :: RuntimeType for Vector4 { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: from_slice (b"struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4)" ,) ; }
    };
}

impl_14!()