#[macro_export]
macro_rules! make_invocation_collector_cfg_ext_trait {
    ($($body:tt)*) => {
        pub trait InvocationCollectorCfgExt<'a, 'b, DRT: DeriveResolutionBound> {
            $($body)*
        }
    };
}