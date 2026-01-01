// SRC: ../rust/compiler/rustc_codegen_cranelift/example/issue-72793.rs
/* AST_META: AST_ID=1 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */
// Adapted from rustc ui test suite (ui/type-alias-impl-trait/issue-72793.rs)

#[feature(type_alias_impl_trait)]

pub trait T {
    type Item;
}
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=S; | COMPLEXITY=4 | LINES=7 */

pub type Alias<'a> = impl T<Item = &'a ()>;

struct S;
impl<'a> T for &'a S {
    type Item = &'a ();
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=filter_positive | COMPLEXITY=2 | LINES=5 */

#[define_opaque(Alias)]
pub fn filter_positive<'a>() -> Alias<'a> {
    &S
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=with_positive | COMPLEXITY=2 | LINES=4 */

fn with_positive(fun: impl Fn(Alias<'_>)) {
    fun(filter_positive());
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=main | COMPLEXITY=2 | LINES=4 */

fn main() {
    with_positive(|_| ());
}