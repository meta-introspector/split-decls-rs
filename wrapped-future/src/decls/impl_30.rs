macro_rules! deps {
    () => {
        AsyncOperationProgressHandler!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < TResult : windows_core :: RuntimeType + 'static , TProgress : windows_core :: RuntimeType + 'static , > windows_core :: RuntimeType for AsyncOperationProgressHandler < TResult , TProgress > { const SIGNATURE : windows_core :: imp :: ConstBuffer = windows_core :: imp :: ConstBuffer :: new () . push_slice (b"pinterface({55690902-0aab-421a-8778-f8ce5026d758}") . push_slice (b";") . push_other (TResult :: SIGNATURE) . push_slice (b";") . push_other (TProgress :: SIGNATURE) . push_slice (b")") ; }
    };
}

impl_30!();