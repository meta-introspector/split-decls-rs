use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Each instance of a type that implements `Label<C>` maps to a
/// unique identifier with respect to `C`, which is used to identify
/// it in the generated .dot file. They can also provide more
/// elaborate (and non-unique) label text that is used in the graphviz
/// rendered output.
/// The graph instance is responsible for providing the DOT compatible
/// identifiers for the nodes and (optionally) rendered labels for the nodes and
/// edges, as well as an identifier for the graph itself.
pub trait Labeller<'a> {
    type Node;
    type Edge;
    /// Must return a DOT compatible identifier naming the graph.
    fn graph_id(&'a self) -> Id<'a>;
    /// Maps `n` to a unique identifier with respect to `self`. The
    /// implementor is responsible for ensuring that the returned name
    /// is a valid DOT identifier.
    fn node_id(&'a self, n: &Self::Node) -> Id<'a>;
    /// Maps `n` to one of the [graphviz `shape` names][1]. If `None`
    /// is returned, no `shape` attribute is specified.
    ///
    /// [1]: https://www.graphviz.org/doc/info/shapes.html
    fn node_shape(&'a self, _node: &Self::Node) -> Option<LabelText<'a>> {
        None
    }
    /// Maps `n` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is just the output from `node_id`.
    fn node_label(&'a self, n: &Self::Node) -> LabelText<'a> {
        LabelStr(self.node_id(n).name)
    }
    /// Maps `e` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is in fact the empty string.
    fn edge_label(&'a self, _e: &Self::Edge) -> LabelText<'a> {
        LabelStr("".into())
    }
    /// Maps `n` to a style that will be used in the rendered output.
    fn node_style(&'a self, _n: &Self::Node) -> Style {
        Style::None
    }
    /// Maps `e` to a style that will be used in the rendered output.
    fn edge_style(&'a self, _e: &Self::Edge) -> Style {
        Style::None
    }
}
