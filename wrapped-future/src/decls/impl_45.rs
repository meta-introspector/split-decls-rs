macro_rules! deps {
    () => {
        AsyncStatus!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl windows_core :: RuntimeType for AsyncStatus { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: from_slice (b"enum(Windows.Foundation.AsyncStatus;i4)") ; }
    };
}

impl_45!();