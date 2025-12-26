#[macro_export]
macro_rules! impl_for_macro_expander {
    ($($body:tt)*) => {
        impl<'a, 'b, DRT: OpaqueDeriveResolution + 'static> MacroExpander<'a, 'b, DRT> {
            $($body)*
        }
    };
}