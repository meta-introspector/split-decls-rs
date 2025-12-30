// Generated macro for LifetimeElisionKind (enum)
macro_rules! Depcrate_lowerLifetimeElisionKind {
() => {
// Module: crate::lower
// Provides: {"LifetimeElisionKind"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum LifetimeElisionKind < 'db > { # [doc = " Create a new anonymous lifetime parameter and reference it."] # [doc = ""] # [doc = " If `report_in_path`, report an error when encountering lifetime elision in a path:"] # [doc = " ```compile_fail"] # [doc = " struct Foo<'a> { x: &'a () }"] # [doc = " async fn foo(x: Foo) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Note: the error should not trigger when the elided lifetime is in a pattern or"] # [doc = " expression-position path:"] # [doc = " ```"] # [doc = " struct Foo<'a> { x: &'a () }"] # [doc = " async fn foo(Foo { x: _ }: Foo<'_>) {}"] # [doc = " ```"] AnonymousCreateParameter { report_in_path : bool } , # [doc = " Replace all anonymous lifetimes by provided lifetime."] Elided (Region < 'db >) , # [doc = " Give a hard error when either `&` or `'_` is written. Used to"] # [doc = " rule out things like `where T: Foo<'_>`. Does not imply an"] # [doc = " error on default object bounds (e.g., `Box<dyn Foo>`)."] AnonymousReportError , # [doc = " Resolves elided lifetimes to `'static` if there are no other lifetimes in scope,"] # [doc = " otherwise give a warning that the previous behavior of introducing a new early-bound"] # [doc = " lifetime is a bug and will be removed (if `only_lint` is enabled)."] StaticIfNoLifetimeInScope { only_lint : bool } , # [doc = " Signal we cannot find which should be the anonymous lifetime."] ElisionFailure , # [doc = " Infer all elided lifetimes."] Infer , }
};
}
