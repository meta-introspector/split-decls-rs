#[macro_export]
macro_rules! make_dummy_visitor_has_attrs_impl {
    ($struct_name:ident, $($body:tt)*) => {
        impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> HasAttrs for $struct_name<'a, 'b, DRT> {
            $($body)*
        }
    };
}