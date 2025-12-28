macro_rules! deps {
    () => {
        Vector2!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl windows_core :: RuntimeType for Vector2 { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: from_slice (b"struct(Windows.Foundation.Numerics.Vector2;f4;f4)" ,) ; }
    };
}

impl_8!()