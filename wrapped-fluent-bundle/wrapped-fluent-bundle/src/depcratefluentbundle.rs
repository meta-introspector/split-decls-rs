// Generated macro for FluentBundle (type)
macro_rules! DepcrateFluentBundle {
() => {
// Module: crate
// Provides: {"FluentBundle"}
// Dependencies: {}
# [doc = " Specialized [`FluentBundle`](crate::bundle::FluentBundle) over"] # [doc = " non-concurrent [`IntlLangMemoizer`](intl_memoizer::IntlLangMemoizer)."] # [doc = ""] # [doc = " This is the basic variant of the [`FluentBundle`](crate::bundle::FluentBundle)."] # [doc = ""] # [doc = " The concurrent specialization can be constructed with"] # [doc = " [`FluentBundle::new_concurrent`](crate::concurrent::FluentBundle::new_concurrent)."] pub type FluentBundle < R > = bundle :: FluentBundle < R , intl_memoizer :: IntlLangMemoizer > ;
};
}
