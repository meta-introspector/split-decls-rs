macro_rules! impl_2 {
    () => {
        impl windows_core :: RuntimeType for AsyncActionCompletedHandler { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: for_interface :: < Self > () ; }
    };
}

impl_2!()