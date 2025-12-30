// Generated macro for FluentBundle (type)
macro_rules! Depcrate_concurrentFluentBundle {
() => {
// Module: crate::concurrent
// Provides: {"FluentBundle"}
// Dependencies: {}
# [doc = " Specialized [`FluentBundle`](crate::bundle::FluentBundle) over"] # [doc = " concurrent [`IntlLangMemoizer`]."] # [doc = ""] # [doc = " A concurrent `FluentBundle` can be constructed with the"] # [doc = " [`FluentBundle::new_concurrent`] method."] # [doc = ""] # [doc = " See [`FluentBundle`](crate::FluentBundle) for the non-concurrent specialization."] pub type FluentBundle < R > = crate :: bundle :: FluentBundle < R , IntlLangMemoizer > ;
};
}
