use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Renders directed graph `g` into the writer `w` in DOT syntax.
/// (Simple wrapper around `render_opts` that passes a default set of options.)
pub fn render<'a, N, E, G, W>(g: &'a G, w: &mut W) -> io::Result<()>
where
    N: Clone + 'a,
    E: Clone + 'a,
    G: Labeller<'a, Node = N, Edge = E> + GraphWalk<'a, Node = N, Edge = E>,
    W: Write,
{
    render_opts(g, w, &[])
}
