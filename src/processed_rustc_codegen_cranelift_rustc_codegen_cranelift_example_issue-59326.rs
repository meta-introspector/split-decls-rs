// SRC: ../rust/compiler/rustc_codegen_cranelift/example/issue-59326.rs
/* AST_META: AST_ID=1 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
// Based on https://github.com/rust-lang/rust/blob/689511047a75a30825e367d4fd45c74604d0b15e/tests/ui/issues/issue-59326.rs#L1
// check-pass
trait Service {
    type S;
}
/* AST_META: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */

trait Framing {
    type F;
}
/* AST_META: AST_ID=3 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=4 */

impl Framing for () {
    type F = ();
}
/* AST_META: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

trait HttpService<F: Framing>: Service<S = F::F> {}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=build_server | COMPLEXITY=2 | LINES=4 */

type BoxService = Box<dyn HttpService<(), S = ()>>;

fn build_server<F: FnOnce() -> BoxService>(_: F) {}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=make_server | COMPLEXITY=2 | LINES=4 */

fn make_server<F: Framing>() -> Box<dyn HttpService<F, S = F::F>> {
    unimplemented!()
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=main | COMPLEXITY=2 | LINES=4 */

fn main() {
    build_server(|| make_server())
}