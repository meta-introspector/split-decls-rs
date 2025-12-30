// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use gloo :: worker :: Spawnable ; use wasm_bindgen_test :: * ; wasm_bindgen_test_configure ! (run_in_browser) ; static MARKDOWN_CONTENT : & str = r#"
## Hello

This content is *rendered* by a **web worker**.

"# ; # [wasm_bindgen_test] async fn markdown_worker_works () { let mut bridge = MarkdownWorker :: spawner () . spawn ("http://127.0.0.1:9999/example_markdown_worker.js") ; let content = bridge . run (MARKDOWN_CONTENT . to_owned ()) . await ; assert_eq ! (& content , r#"<h2>Hello</h2>
<p>This content is <em>rendered</em> by a <strong>web worker</strong>.</p>
"#) ; } }
};
}
