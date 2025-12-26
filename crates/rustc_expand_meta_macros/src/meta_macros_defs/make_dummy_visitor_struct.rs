#[macro_export]
macro_rules! make_dummy_visitor_struct {
    ($($body:tt)*) => {
        pub struct DummyVisitor<'a, 'b, DRT: OpaqueDeriveResolution + 'static> {
            $($body)*
        }
    };
}