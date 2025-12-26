#[macro_export]
macro_rules! make_invocation_collector_struct {
    ($($body:tt)*) => {
        pub struct InvocationCollector<'a, 'b, DRT: DeriveResolutionProvider> {
            $($body)*
        }
    };
}