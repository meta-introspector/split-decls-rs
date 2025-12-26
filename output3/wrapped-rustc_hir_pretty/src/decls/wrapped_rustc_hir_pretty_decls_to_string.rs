use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn to_string<F>(ann: &dyn PpAnn, f: F) -> String
where
    F: FnOnce(&mut State<'_>),
{
    let mut printer = State {
        s: pp::Printer::new(),
        comments: None,
        attrs: &|_| &[],
        ann,
    };
    f(&mut printer);
    printer.s.eof()
}
