#[macro_export]
macro_rules! make_invocation_mut_visitor_impl {
    ($struct_name:ident, $($body:tt)*) => {
        impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> InvocationMutVisitor for $struct_name<'a, 'b, DRT> {
            $($body)*
        }
    };
}