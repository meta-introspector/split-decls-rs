#[macro_export]
macro_rules! make_dummy_visitor_has_node_id_impl {
    ($struct_name:ident, $($body:tt)*) => {
        impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> HasNodeId for $struct_name<'a, 'b, DRT> {
            $($body)*
        }
    };
}