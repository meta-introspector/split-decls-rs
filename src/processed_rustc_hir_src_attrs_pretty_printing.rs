// SRC: ../rust/compiler/rustc_hir/src/attrs/pretty_printing.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */
use std::num::NonZero;

use crate::rustc_abi::Align;
use crate::rustc_complete::token::CommentKind;
use crate::rustc_complete::{AttrStyle, IntTy, UintTy};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
use rustc_ast_pretty::pp::Printer;
use crate::rustc_complete::hygiene::Transparency;
use crate::rustc_complete::{ErrorGuaranteed, Ident, Span, Symbol};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=5 | LINES=19 */
use crate::rustc_target::spec::SanitizerSet;
use thin_vec::ThinVec;

use crate::limit::Limit;

/// This trait is used to print attributes in `rustc_hir_pretty`.
///
/// For structs and enums it can be derived using [`rustc_macros::PrintAttribute`].
/// The output will look a lot like a `Debug` implementation, but fields of several types
/// like [`Span`]s and empty tuples, are gracefully skipped so they don't clutter the
/// representation much.
pub trait PrintAttribute {
    /// Whether or not this will render as something meaningful, or if it's skipped
    /// (which will force the containing struct to also skip printing a comma
    /// and the field name).
    fn should_render(&self) -> bool;

    fn print_attribute(&self, p: &mut Printer);
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=6 | LINES=10 */

impl PrintAttribute for u128 {
    fn should_render(&self) -> bool {
        true
    }

    fn print_attribute(&self, p: &mut Printer) {
        p.word(self.to_string())
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=6 | LINES=10 */

impl<T: PrintAttribute> PrintAttribute for &T {
    fn should_render(&self) -> bool {
        T::should_render(self)
    }

    fn print_attribute(&self, p: &mut Printer) {
        T::print_attribute(self, p)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=9 | LINES=11 */
impl<T: PrintAttribute> PrintAttribute for Option<T> {
    fn should_render(&self) -> bool {
        self.as_ref().is_some_and(|x| x.should_render())
    }

    fn print_attribute(&self, p: &mut Printer) {
        if let Some(i) = self {
            T::print_attribute(i, p)
        }
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=12 | LINES=18 */
impl<T: PrintAttribute> PrintAttribute for ThinVec<T> {
    fn should_render(&self) -> bool {
        self.is_empty() || self[0].should_render()
    }

    fn print_attribute(&self, p: &mut Printer) {
        let mut last_printed = false;
        p.word("[");
        for i in self {
            if last_printed {
                p.word_space(",");
            }
            i.print_attribute(p);
            last_printed = i.should_render();
        }
        p.word("]");
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=13 | LINES=8 */
macro_rules! print_skip {
    ($($t: ty),* $(,)?) => {$(
        impl PrintAttribute for $t {
            fn should_render(&self) -> bool { false }
            fn print_attribute(&self, _: &mut Printer) { }
        })*
    };
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=14 | LINES=11 */

macro_rules! print_disp {
    ($($t: ty),* $(,)?) => {$(
        impl PrintAttribute for $t {
            fn should_render(&self) -> bool { true }
            fn print_attribute(&self, p: &mut Printer) {
                p.word(format!("{}", self));
            }
        }
    )*};
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=14 | LINES=10 */
macro_rules! print_debug {
    ($($t: ty),* $(,)?) => {$(
        impl PrintAttribute for $t {
            fn should_render(&self) -> bool { true }
            fn print_attribute(&self, p: &mut Printer) {
                p.word(format!("{:?}", self));
            }
        }
    )*};
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=should_render | COMPLEXITY=29 | LINES=42 */

macro_rules! print_tup {
    (num_should_render $($ts: ident)*) => { 0 $(+ $ts.should_render() as usize)* };
    () => {};
    ($t: ident $($ts: ident)*) => {
        #[allow(non_snake_case, unused)]
        impl<$t: PrintAttribute, $($ts: PrintAttribute),*> PrintAttribute for ($t, $($ts),*) {
            fn should_render(&self) -> bool {
                let ($t, $($ts),*) = self;
                print_tup!(num_should_render $t $($ts)*) != 0
            }

            fn print_attribute(&self, p: &mut Printer) {
                let ($t, $($ts),*) = self;
                let parens = print_tup!(num_should_render $t $($ts)*) > 1;
                if parens {
                    p.popen();
                }

                let mut printed_anything = $t.should_render();

                $t.print_attribute(p);

                $(
                    if $ts.should_render() {
                        if printed_anything {
                            p.word_space(",");
                        }
                        printed_anything = true;
                    }
                    $ts.print_attribute(p);
                )*

                if parens {
                    p.pclose();
                }
            }
        }

        print_tup!($($ts)*);
    };
}
/* AST_META: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=15 */

print_tup!(A B C D E F G H);
print_skip!(Span, (), ErrorGuaranteed);
print_disp!(u16, bool, NonZero<u32>, Limit);
print_debug!(
    Symbol,
    Ident,
    UintTy,
    IntTy,
    Align,
    AttrStyle,
    CommentKind,
    Transparency,
    SanitizerSet,
);