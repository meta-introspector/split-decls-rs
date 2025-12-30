// Generated macro for unstable (macro)
macro_rules! Depcrateunstable {
() => {
// Module: crate
// Provides: {"unstable"}
// Dependencies: {}
# [cfg (feature = "unstable")] macro_rules ! unstable { ($ feature : ident , $ issue : literal) => { concat ! (r#"<div class="stab unstable">"# , r#"<span class="emoji">🔬</span>"# , r#"<span>This is a nightly-only experimental API. (<code>"# , stringify ! ($ feature) , r#"</code>&nbsp;<a href="https://github.com/rust-lang/rust/issues/"# , $ issue , r#"">#"# , $ issue , r#"</a>)</span>"# , r#"</div>"#) } ; }
};
}
