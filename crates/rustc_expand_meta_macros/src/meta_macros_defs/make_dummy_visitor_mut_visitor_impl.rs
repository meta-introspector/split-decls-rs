#[macro_export]
macro_rules! make_dummy_visitor_mut_visitor_impl {
    ($struct_name:ident, $($body:tt)*) => {
        impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> MutVisitor for $struct_name<'a, 'b, DRT> {
            $($body)*
        }
    };
}