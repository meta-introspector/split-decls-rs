// Generated macro for impl_702 (impl)
macro_rules! Depcrate_personnames_providerimpl_702 {
() => {
// Module: crate::personnames::provider
// Provides: {"impl_702"}
// Dependencies: {}
impl From < FormattingOrder > for PersonNamesFormattingAttributes { fn from (value : FormattingOrder) -> Self { match value { FormattingOrder :: GivenFirst => PersonNamesFormattingAttributes :: GivenFirst , FormattingOrder :: SurnameFirst => PersonNamesFormattingAttributes :: SurnameFirst , FormattingOrder :: Sorting => PersonNamesFormattingAttributes :: Sorting , } } }
};
}
