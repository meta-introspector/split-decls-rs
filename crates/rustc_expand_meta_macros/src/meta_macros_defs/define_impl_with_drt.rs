#[macro_export]
macro_rules! DefineImplWithDRT {
    ($trait_name:path, $type_name:path, { $($body:tt)* }) => {
        impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> $trait_name for $type_name<'a, 'b, DRT> {
            $($body)*
        }
    };
}