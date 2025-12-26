#[macro_export]
macro_rules! make_invocation_collector_cfg_ext_impl {
    ($struct_name:ident, $($body:tt)*) => {
        impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> InvocationCollectorCfgExt<'a, 'b, DRT> for $struct_name<'a, 'b, DRT> {
            $($body)*
        }
    };
}