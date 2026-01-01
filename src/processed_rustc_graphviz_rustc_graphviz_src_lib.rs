/* FP:lib.rs-0001 */ // Generate files suitable for use with [Graphviz](https://www.graphviz.org/)
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // The `render` function generates output (e.g., an `output.dot` file) for
/* FP:lib.rs-0004 */ // use with [Graphviz](https://www.graphviz.org/) by walking a labeled
/* FP:lib.rs-0005 */ // graph. (Graphviz can then automatically lay out the nodes and edges
/* FP:lib.rs-0006 */ // of the graph, and also optionally render the graph as an image or
/* FP:lib.rs-0007 */ // other [output formats](https://www.graphviz.org/docs/outputs), such as SVG.)
/* FP:lib.rs-0008 */ //
/* FP:lib.rs-0009 */ // Rather than impose some particular graph data structure on clients,
/* FP:lib.rs-0010 */ // this library exposes two traits that clients can implement on their
/* FP:lib.rs-0011 */ // own structs before handing them over to the rendering function.
/* FP:lib.rs-0012 */ //
/* FP:lib.rs-0013 */ // Note: This library does not yet provide access to the full
/* FP:lib.rs-0014 */ // expressiveness of the [DOT language](https://www.graphviz.org/doc/info/lang.html).
/* FP:lib.rs-0015 */ // For example, there are many [attributes](https://www.graphviz.org/doc/info/attrs.html)
/* FP:lib.rs-0016 */ // related to providing layout hints (e.g., left-to-right versus top-down, which
/* FP:lib.rs-0017 */ // algorithm to use, etc). The current intention of this library is to
/* FP:lib.rs-0018 */ // emit a human-readable .dot file with very regular structure suitable
/* FP:lib.rs-0019 */ // for easy post-processing.
/* FP:lib.rs-0020 */ //
/* FP:lib.rs-0021 */ // # Examples
/* FP:lib.rs-0022 */ //
/* FP:lib.rs-0023 */ // The first example uses a very simple graph representation: a list of
/* FP:lib.rs-0024 */ // pairs of ints, representing the edges (the node set is implicit).
/* FP:lib.rs-0025 */ // Each node label is derived directly from the int representing the node,
/* FP:lib.rs-0026 */ // while the edge labels are all empty strings.
/* FP:lib.rs-0027 */ //
/* FP:lib.rs-0028 */ // This example also illustrates how to use `Cow<[T]>` to return
/* FP:lib.rs-0029 */ // an owned vector or a borrowed slice as appropriate: we construct the
/* FP:lib.rs-0030 */ // node vector from scratch, but borrow the edge list (rather than
/* FP:lib.rs-0031 */ // constructing a copy of all the edges from scratch).
/* FP:lib.rs-0032 */ //
/* FP:lib.rs-0033 */ // The output from this example renders five nodes, with the first four
/* FP:lib.rs-0034 */ // forming a diamond-shaped acyclic graph and then pointing to the fifth
/* FP:lib.rs-0035 */ // which is cyclic.
/* FP:lib.rs-0036 */ //
/* FP:lib.rs-0037 */ // ```rust
/* FP:lib.rs-0038 */ // #[feature(rustc_private)]
/* FP:lib.rs-0039 */ //
/* FP:lib.rs-0040 */ // use std::io::Write;
/* FP:lib.rs-0041 */ // use rustc_graphviz as dot;
/* FP:lib.rs-0042 */ //
/* FP:lib.rs-0043 */ // type Nd = isize;
/* FP:lib.rs-0044 */ // type Ed = (isize,isize);
/* FP:lib.rs-0045 */ // struct Edges(Vec<Ed>);
/* FP:lib.rs-0046 */ //
/* FP:lib.rs-0047 */ // pub fn render_to<W: Write>(output: &mut W) {
/* FP:lib.rs-0048 */ //     let edges = Edges(vec![(0,1), (0,2), (1,3), (2,3), (3,4), (4,4)]);
/* FP:lib.rs-0049 */ //     dot::render(&edges, output).unwrap()
/* FP:lib.rs-0050 */ // }
/* FP:lib.rs-0051 */ //
/* FP:lib.rs-0052 */ // impl<'a> dot::Labeller<'a> for Edges {
/* FP:lib.rs-0053 */ //     type Node = Nd;
/* FP:lib.rs-0054 */ //     type Edge = Ed;
/* FP:lib.rs-0055 */ //     fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example1").unwrap() }
/* FP:lib.rs-0056 */ //
/* FP:lib.rs-0057 */ //     fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
/* FP:lib.rs-0058 */ //         dot::Id::new(format!("N{}", *n)).unwrap()
/* FP:lib.rs-0059 */ //     }
/* FP:lib.rs-0060 */ // }
/* FP:lib.rs-0061 */ //
/* FP:lib.rs-0062 */ // impl<'a> dot::GraphWalk<'a> for Edges {
/* FP:lib.rs-0063 */ //     type Node = Nd;
/* FP:lib.rs-0064 */ //     type Edge = Ed;
/* FP:lib.rs-0065 */ //     fn nodes(&self) -> dot::Nodes<'a,Nd> {
/* FP:lib.rs-0066 */ //         // (assumes that |N| \approxeq |E|)
/* FP:lib.rs-0067 */ //         let &Edges(ref v) = self;
/* FP:lib.rs-0068 */ //         let mut nodes = Vec::with_capacity(v.len());
/* FP:lib.rs-0069 */ //         for &(s,t) in v {
/* FP:lib.rs-0070 */ //             nodes.push(s); nodes.push(t);
/* FP:lib.rs-0071 */ //         }
/* FP:lib.rs-0072 */ //         nodes.sort();
/* FP:lib.rs-0073 */ //         nodes.dedup();
/* FP:lib.rs-0074 */ //         nodes.into()
/* FP:lib.rs-0075 */ //     }
/* FP:lib.rs-0076 */ //
/* FP:lib.rs-0077 */ //     fn edges(&'a self) -> dot::Edges<'a,Ed> {
/* FP:lib.rs-0078 */ //         let &Edges(ref edges) = self;
/* FP:lib.rs-0079 */ //         (&edges[..]).into()
/* FP:lib.rs-0080 */ //     }
/* FP:lib.rs-0081 */ //
/* FP:lib.rs-0082 */ //     fn source(&self, e: &Ed) -> Nd { let &(s,_) = e; s }
/* FP:lib.rs-0083 */ //
/* FP:lib.rs-0084 */ //     fn target(&self, e: &Ed) -> Nd { let &(_,t) = e; t }
/* FP:lib.rs-0085 */ // }
/* FP:lib.rs-0086 */ //
/* FP:lib.rs-0087 */ // # pub fn main() { render_to(&mut Vec::new()) }
/* FP:lib.rs-0088 */ // ```
/* FP:lib.rs-0089 */ //
/* FP:lib.rs-0090 */ // ```no_run
/* FP:lib.rs-0091 */ // # pub fn render_to<W:std::io::Write>(output: &mut W) { unimplemented!() }
/* FP:lib.rs-0092 */ // pub fn main() {
/* FP:lib.rs-0093 */ //     use std::fs::File;
/* FP:lib.rs-0094 */ //     let mut f = File::create("example1.dot").unwrap();
/* FP:lib.rs-0095 */ //     render_to(&mut f)
/* FP:lib.rs-0096 */ // }
/* FP:lib.rs-0097 */ // ```
/* FP:lib.rs-0098 */ //
/* FP:lib.rs-0099 */ // Output from first example (in `example1.dot`):
/* FP:lib.rs-0100 */ //
/* FP:lib.rs-0101 */ // ```dot
/* FP:lib.rs-0102 */ // digraph example1 {
/* FP:lib.rs-0103 */ //     N0[label="N0"];
/* FP:lib.rs-0104 */ //     N1[label="N1"];
/* FP:lib.rs-0105 */ //     N2[label="N2"];
/* FP:lib.rs-0106 */ //     N3[label="N3"];
/* FP:lib.rs-0107 */ //     N4[label="N4"];
/* FP:lib.rs-0108 */ //     N0 -> N1[label=""];
/* FP:lib.rs-0109 */ //     N0 -> N2[label=""];
/* FP:lib.rs-0110 */ //     N1 -> N3[label=""];
/* FP:lib.rs-0111 */ //     N2 -> N3[label=""];
/* FP:lib.rs-0112 */ //     N3 -> N4[label=""];
/* FP:lib.rs-0113 */ //     N4 -> N4[label=""];
/* FP:lib.rs-0114 */ // }
/* FP:lib.rs-0115 */ // ```
/* FP:lib.rs-0116 */ //
/* FP:lib.rs-0117 */ // The second example illustrates using `node_label` and `edge_label` to
/* FP:lib.rs-0118 */ // add labels to the nodes and edges in the rendered graph. The graph
/* FP:lib.rs-0119 */ // here carries both `nodes` (the label text to use for rendering a
/* FP:lib.rs-0120 */ // particular node), and `edges` (again a list of `(source,target)`
/* FP:lib.rs-0121 */ // indices).
/* FP:lib.rs-0122 */ //
/* FP:lib.rs-0123 */ // This example also illustrates how to use a type (in this case the edge
/* FP:lib.rs-0124 */ // type) that shares substructure with the graph: the edge type here is a
/* FP:lib.rs-0125 */ // direct reference to the `(source,target)` pair stored in the graph's
/* FP:lib.rs-0126 */ // internal vector (rather than passing around a copy of the pair
/* FP:lib.rs-0127 */ // itself). Note that this implies that `fn edges(&'a self)` must
/* FP:lib.rs-0128 */ // construct a fresh `Vec<&'a (usize,usize)>` from the `Vec<(usize,usize)>`
/* FP:lib.rs-0129 */ // edges stored in `self`.
/* FP:lib.rs-0130 */ //
/* FP:lib.rs-0131 */ // Since both the set of nodes and the set of edges are always
/* FP:lib.rs-0132 */ // constructed from scratch via iterators, we use the `collect()` method
/* FP:lib.rs-0133 */ // from the `Iterator` trait to collect the nodes and edges into freshly
/* FP:lib.rs-0134 */ // constructed growable `Vec` values (rather than using `Cow` as in the
/* FP:lib.rs-0135 */ // first example above).
/* FP:lib.rs-0136 */ //
/* FP:lib.rs-0137 */ // The output from this example renders four nodes that make up the
/* FP:lib.rs-0138 */ // Hasse-diagram for the subsets of the set `{x, y}`. Each edge is
/* FP:lib.rs-0139 */ // labeled with the &sube; character (specified using the HTML character
/* FP:lib.rs-0140 */ // entity `&sube`).
/* FP:lib.rs-0141 */ //
/* FP:lib.rs-0142 */ // ```rust
/* FP:lib.rs-0143 */ // #[feature(rustc_private)]
/* FP:lib.rs-0144 */ //
/* FP:lib.rs-0145 */ // use std::io::Write;
/* FP:lib.rs-0146 */ // use rustc_graphviz as dot;
/* FP:lib.rs-0147 */ //
/* FP:lib.rs-0148 */ // type Nd = usize;
/* FP:lib.rs-0149 */ // type Ed<'a> = &'a (usize, usize);
/* FP:lib.rs-0150 */ // struct Graph { nodes: Vec<&'static str>, edges: Vec<(usize,usize)> }
/* FP:lib.rs-0151 */ //
/* FP:lib.rs-0152 */ // pub fn render_to<W: Write>(output: &mut W) {
/* FP:lib.rs-0153 */ //     let nodes = vec!["{x,y}","{x}","{y}","{}"];
/* FP:lib.rs-0154 */ //     let edges = vec![(0,1), (0,2), (1,3), (2,3)];
/* FP:lib.rs-0155 */ //     let graph = Graph { nodes: nodes, edges: edges };
/* FP:lib.rs-0156 */ //
/* FP:lib.rs-0157 */ //     dot::render(&graph, output).unwrap()
/* FP:lib.rs-0158 */ // }
/* FP:lib.rs-0159 */ //
/* FP:lib.rs-0160 */ // impl<'a> dot::Labeller<'a> for Graph {
/* FP:lib.rs-0161 */ //     type Node = Nd;
/* FP:lib.rs-0162 */ //     type Edge = Ed<'a>;
/* FP:lib.rs-0163 */ //     fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example2").unwrap() }
/* FP:lib.rs-0164 */ //     fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
/* FP:lib.rs-0165 */ //         dot::Id::new(format!("N{}", n)).unwrap()
/* FP:lib.rs-0166 */ //     }
/* FP:lib.rs-0167 */ //     fn node_label(&self, n: &Nd) -> dot::LabelText<'_> {
/* FP:lib.rs-0168 */ //         dot::LabelText::LabelStr(self.nodes[*n].into())
/* FP:lib.rs-0169 */ //     }
/* FP:lib.rs-0170 */ //     fn edge_label(&self, _: &Ed<'_>) -> dot::LabelText<'_> {
/* FP:lib.rs-0171 */ //         dot::LabelText::LabelStr("&sube;".into())
/* FP:lib.rs-0172 */ //     }
/* FP:lib.rs-0173 */ // }
/* FP:lib.rs-0174 */ //
/* FP:lib.rs-0175 */ // impl<'a> dot::GraphWalk<'a> for Graph {
/* FP:lib.rs-0176 */ //     type Node = Nd;
/* FP:lib.rs-0177 */ //     type Edge = Ed<'a>;
/* FP:lib.rs-0178 */ //     fn nodes(&self) -> dot::Nodes<'a,Nd> { (0..self.nodes.len()).collect() }
/* FP:lib.rs-0179 */ //     fn edges(&'a self) -> dot::Edges<'a,Ed<'a>> { self.edges.iter().collect() }
/* FP:lib.rs-0180 */ //     fn source(&self, e: &Ed<'_>) -> Nd { let & &(s,_) = e; s }
/* FP:lib.rs-0181 */ //     fn target(&self, e: &Ed<'_>) -> Nd { let & &(_,t) = e; t }
/* FP:lib.rs-0182 */ // }
/* FP:lib.rs-0183 */ //
/* FP:lib.rs-0184 */ // # pub fn main() { render_to(&mut Vec::new()) }
/* FP:lib.rs-0185 */ // ```
/* FP:lib.rs-0186 */ //
/* FP:lib.rs-0187 */ // ```no_run
/* FP:lib.rs-0188 */ // # pub fn render_to<W:std::io::Write>(output: &mut W) { unimplemented!() }
/* FP:lib.rs-0189 */ // pub fn main() {
/* FP:lib.rs-0190 */ //     use std::fs::File;
/* FP:lib.rs-0191 */ //     let mut f = File::create("example2.dot").unwrap();
/* FP:lib.rs-0192 */ //     render_to(&mut f)
/* FP:lib.rs-0193 */ // }
/* FP:lib.rs-0194 */ // ```
/* FP:lib.rs-0195 */ //
/* FP:lib.rs-0196 */ // The third example is similar to the second, except now each node and
/* FP:lib.rs-0197 */ // edge now carries a reference to the string label for each node as well
/* FP:lib.rs-0198 */ // as that node's index. (This is another illustration of how to share
/* FP:lib.rs-0199 */ // structure with the graph itself, and why one might want to do so.)
/* FP:lib.rs-0200 */ //
/* FP:lib.rs-0201 */ // The output from this example is the same as the second example: the
/* FP:lib.rs-0202 */ // Hasse-diagram for the subsets of the set `{x, y}`.
/* FP:lib.rs-0203 */ //
/* FP:lib.rs-0204 */ // ```rust
/* FP:lib.rs-0205 */ // #[feature(rustc_private)]
/* FP:lib.rs-0206 */ //
/* FP:lib.rs-0207 */ // use std::io::Write;
/* FP:lib.rs-0208 */ // use rustc_graphviz as dot;
/* FP:lib.rs-0209 */ //
/* FP:lib.rs-0210 */ // type Nd<'a> = (usize, &'a str);
/* FP:lib.rs-0211 */ // type Ed<'a> = (Nd<'a>, Nd<'a>);
/* FP:lib.rs-0212 */ // struct Graph { nodes: Vec<&'static str>, edges: Vec<(usize,usize)> }
/* FP:lib.rs-0213 */ //
/* FP:lib.rs-0214 */ // pub fn render_to<W: Write>(output: &mut W) {
/* FP:lib.rs-0215 */ //     let nodes = vec!["{x,y}","{x}","{y}","{}"];
/* FP:lib.rs-0216 */ //     let edges = vec![(0,1), (0,2), (1,3), (2,3)];
/* FP:lib.rs-0217 */ //     let graph = Graph { nodes: nodes, edges: edges };
/* FP:lib.rs-0218 */ //
/* FP:lib.rs-0219 */ //     dot::render(&graph, output).unwrap()
/* FP:lib.rs-0220 */ // }
/* FP:lib.rs-0221 */ //
/* FP:lib.rs-0222 */ // impl<'a> dot::Labeller<'a> for Graph {
/* FP:lib.rs-0223 */ //     type Node = Nd<'a>;
/* FP:lib.rs-0224 */ //     type Edge = Ed<'a>;
/* FP:lib.rs-0225 */ //     fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example3").unwrap() }
/* FP:lib.rs-0226 */ //     fn node_id(&'a self, n: &Nd<'a>) -> dot::Id<'a> {
/* FP:lib.rs-0227 */ //         dot::Id::new(format!("N{}", n.0)).unwrap()
/* FP:lib.rs-0228 */ //     }
/* FP:lib.rs-0229 */ //     fn node_label(&self, n: &Nd<'_>) -> dot::LabelText<'_> {
/* FP:lib.rs-0230 */ //         let &(i, _) = n;
/* FP:lib.rs-0231 */ //         dot::LabelText::LabelStr(self.nodes[i].into())
/* FP:lib.rs-0232 */ //     }
/* FP:lib.rs-0233 */ //     fn edge_label(&self, _: &Ed<'_>) -> dot::LabelText<'_> {
/* FP:lib.rs-0234 */ //         dot::LabelText::LabelStr("&sube;".into())
/* FP:lib.rs-0235 */ //     }
/* FP:lib.rs-0236 */ // }
/* FP:lib.rs-0237 */ //
/* FP:lib.rs-0238 */ // impl<'a> dot::GraphWalk<'a> for Graph {
/* FP:lib.rs-0239 */ //     type Node = Nd<'a>;
/* FP:lib.rs-0240 */ //     type Edge = Ed<'a>;
/* FP:lib.rs-0241 */ //     fn nodes(&'a self) -> dot::Nodes<'a,Nd<'a>> {
/* FP:lib.rs-0242 */ //         self.nodes.iter().map(|s| &s[..]).enumerate().collect()
/* FP:lib.rs-0243 */ //     }
/* FP:lib.rs-0244 */ //     fn edges(&'a self) -> dot::Edges<'a,Ed<'a>> {
/* FP:lib.rs-0245 */ //         self.edges.iter()
/* FP:lib.rs-0246 */ //             .map(|&(i,j)|((i, &self.nodes[i][..]),
/* FP:lib.rs-0247 */ //                           (j, &self.nodes[j][..])))
/* FP:lib.rs-0248 */ //             .collect()
/* FP:lib.rs-0249 */ //     }
/* FP:lib.rs-0250 */ //     fn source(&self, e: &Ed<'a>) -> Nd<'a> { let &(s,_) = e; s }
/* FP:lib.rs-0251 */ //     fn target(&self, e: &Ed<'a>) -> Nd<'a> { let &(_,t) = e; t }
/* FP:lib.rs-0252 */ // }
/* FP:lib.rs-0253 */ //
/* FP:lib.rs-0254 */ // # pub fn main() { render_to(&mut Vec::new()) }
/* FP:lib.rs-0255 */ // ```
/* FP:lib.rs-0256 */ //
/* FP:lib.rs-0257 */ // ```no_run
/* FP:lib.rs-0258 */ // # pub fn render_to<W:std::io::Write>(output: &mut W) { unimplemented!() }
/* FP:lib.rs-0259 */ // pub fn main() {
/* FP:lib.rs-0260 */ //     use std::fs::File;
/* FP:lib.rs-0261 */ //     let mut f = File::create("example3.dot").unwrap();
/* FP:lib.rs-0262 */ //     render_to(&mut f)
/* FP:lib.rs-0263 */ // }
/* FP:lib.rs-0264 */ // ```
/* FP:lib.rs-0265 */ //
/* FP:lib.rs-0266 */ // # References
/* FP:lib.rs-0267 */ //
/* FP:lib.rs-0268 */ // * [Graphviz](https://www.graphviz.org/)
/* FP:lib.rs-0269 */ //
/* FP:lib.rs-0270 */ // * [DOT language](https://www.graphviz.org/doc/info/lang.html)
/* FP:lib.rs-0271 */ 
/* FP:lib.rs-0272 */ // tidy-alphabetical-start
/* FP:lib.rs-0273 */ #[allow(internal_features)]
/* FP:lib.rs-0274 */ #[doc(
/* FP:lib.rs-0275 */     html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/",
/* FP:lib.rs-0276 */     test(attr(allow(unused_variables), deny(warnings)))
/* FP:lib.rs-0277 */ )]
/* FP:lib.rs-0278 */ #[doc(rust_logo)]
/* FP:lib.rs-0279 */ #[feature(rustdoc_internals)]
/* FP:lib.rs-0280 */ // tidy-alphabetical-end
/* FP:lib.rs-0281 */ 
/* FP:lib.rs-0282 */ use std::borrow::Cow;
/* FP:lib.rs-0283 */ use std::io;
/* FP:lib.rs-0284 */ use std::io::prelude::*;
/* FP:lib.rs-0285 */ 
/* FP:lib.rs-0286 */ use LabelText::*;
/* FP:lib.rs-0287 */ 
/* FP:lib.rs-0288 */ /// The text for a graphviz label on a node or edge.
/* FP:lib.rs-0289 */ pub enum LabelText<'a> {
/* FP:lib.rs-0290 */     /// This kind of label preserves the text directly as is.
/* FP:lib.rs-0291 */     ///
/* FP:lib.rs-0292 */     /// Occurrences of backslashes (`\`) are escaped, and thus appear
/* FP:lib.rs-0293 */     /// as backslashes in the rendered label.
/* FP:lib.rs-0294 */     LabelStr(Cow<'a, str>),
/* FP:lib.rs-0295 */ 
/* FP:lib.rs-0296 */     /// This kind of label uses the graphviz label escString type:
/* FP:lib.rs-0297 */     /// <https://www.graphviz.org/docs/attr-types/escString>
/* FP:lib.rs-0298 */     ///
/* FP:lib.rs-0299 */     /// Occurrences of backslashes (`\`) are not escaped; instead they
/* FP:lib.rs-0300 */     /// are interpreted as initiating an escString escape sequence.
/* FP:lib.rs-0301 */     ///
/* FP:lib.rs-0302 */     /// Escape sequences of particular interest: in addition to `\n`
/* FP:lib.rs-0303 */     /// to break a line (centering the line preceding the `\n`), there
/* FP:lib.rs-0304 */     /// are also the escape sequences `\l` which left-justifies the
/* FP:lib.rs-0305 */     /// preceding line and `\r` which right-justifies it.
/* FP:lib.rs-0306 */     EscStr(Cow<'a, str>),
/* FP:lib.rs-0307 */ 
/* FP:lib.rs-0308 */     /// This uses a graphviz [HTML string label][html]. The string is
/* FP:lib.rs-0309 */     /// printed exactly as given, but between `<` and `>`. **No
/* FP:lib.rs-0310 */     /// escaping is performed.**
/* FP:lib.rs-0311 */     ///
/* FP:lib.rs-0312 */     /// [html]: https://www.graphviz.org/doc/info/shapes.html#html
/* FP:lib.rs-0313 */     HtmlStr(Cow<'a, str>),
/* FP:lib.rs-0314 */ }
/* FP:lib.rs-0315 */ 
/* FP:lib.rs-0316 */ /// The style for a node or edge.
/* FP:lib.rs-0317 */ /// See <https://www.graphviz.org/docs/attr-types/style/> for descriptions.
/* FP:lib.rs-0318 */ /// Note that some of these are not valid for edges.
/* FP:lib.rs-0319 */ #[derive(Copy, Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-0320 */ pub enum Style {
/* FP:lib.rs-0321 */     None,
/* FP:lib.rs-0322 */     Solid,
/* FP:lib.rs-0323 */     Dashed,
/* FP:lib.rs-0324 */     Dotted,
/* FP:lib.rs-0325 */     Bold,
/* FP:lib.rs-0326 */     Rounded,
/* FP:lib.rs-0327 */     Diagonals,
/* FP:lib.rs-0328 */     Filled,
/* FP:lib.rs-0329 */     Striped,
/* FP:lib.rs-0330 */     Wedged,
/* FP:lib.rs-0331 */ }
/* FP:lib.rs-0332 */ 
/* FP:lib.rs-0333 */ impl Style {
/* FP:lib.rs-0334 */     pub fn as_slice(self) -> &'static str {
/* FP:lib.rs-0335 */         match self {
/* FP:lib.rs-0336 */             Style::None => "",
/* FP:lib.rs-0337 */             Style::Solid => "solid",
/* FP:lib.rs-0338 */             Style::Dashed => "dashed",
/* FP:lib.rs-0339 */             Style::Dotted => "dotted",
/* FP:lib.rs-0340 */             Style::Bold => "bold",
/* FP:lib.rs-0341 */             Style::Rounded => "rounded",
/* FP:lib.rs-0342 */             Style::Diagonals => "diagonals",
/* FP:lib.rs-0343 */             Style::Filled => "filled",
/* FP:lib.rs-0344 */             Style::Striped => "striped",
/* FP:lib.rs-0345 */             Style::Wedged => "wedged",
/* FP:lib.rs-0346 */         }
/* FP:lib.rs-0347 */     }
/* FP:lib.rs-0348 */ }
/* FP:lib.rs-0349 */ 
/* FP:lib.rs-0350 */ // There is a tension in the design of the labelling API.
/* FP:lib.rs-0351 */ //
/* FP:lib.rs-0352 */ // For example, I considered making a `Labeller<T>` trait that
/* FP:lib.rs-0353 */ // provides labels for `T`, and then making the graph type `G`
/* FP:lib.rs-0354 */ // implement `Labeller<Node>` and `Labeller<Edge>`. However, this is
/* FP:lib.rs-0355 */ // not possible without functional dependencies. (One could work
/* FP:lib.rs-0356 */ // around that, but I did not explore that avenue heavily.)
/* FP:lib.rs-0357 */ //
/* FP:lib.rs-0358 */ // Another approach that I actually used for a while was to make a
/* FP:lib.rs-0359 */ // `Label<Context>` trait that is implemented by the client-specific
/* FP:lib.rs-0360 */ // Node and Edge types (as well as an implementation on Graph itself
/* FP:lib.rs-0361 */ // for the overall name for the graph). The main disadvantage of this
/* FP:lib.rs-0362 */ // second approach (compared to having the `G` type parameter
/* FP:lib.rs-0363 */ // implement a Labelling service) that I have encountered is that it
/* FP:lib.rs-0364 */ // makes it impossible to use types outside of the current crate
/* FP:lib.rs-0365 */ // directly as Nodes/Edges; you need to wrap them in newtype'd
/* FP:lib.rs-0366 */ // structs. See e.g., the `No` and `Ed` structs in the examples. (In
/* FP:lib.rs-0367 */ // practice clients using a graph in some other crate would need to
/* FP:lib.rs-0368 */ // provide some sort of adapter shim over the graph anyway to
/* FP:lib.rs-0369 */ // interface with this library).
/* FP:lib.rs-0370 */ //
/* FP:lib.rs-0371 */ // Another approach would be to make a single `Labeller<N,E>` trait
/* FP:lib.rs-0372 */ // that provides three methods (graph_label, node_label, edge_label),
/* FP:lib.rs-0373 */ // and then make `G` implement `Labeller<N,E>`. At first this did not
/* FP:lib.rs-0374 */ // appeal to me, since I had thought I would need separate methods on
/* FP:lib.rs-0375 */ // each data variant for dot-internal identifiers versus user-visible
/* FP:lib.rs-0376 */ // labels. However, the identifier/label distinction only arises for
/* FP:lib.rs-0377 */ // nodes; graphs themselves only have identifiers, and edges only have
/* FP:lib.rs-0378 */ // labels.
/* FP:lib.rs-0379 */ //
/* FP:lib.rs-0380 */ // So in the end I decided to use the third approach described above.
/* FP:lib.rs-0381 */ 
/* FP:lib.rs-0382 */ /// `Id` is a Graphviz `ID`.
/* FP:lib.rs-0383 */ pub struct Id<'a> {
/* FP:lib.rs-0384 */     name: Cow<'a, str>,
/* FP:lib.rs-0385 */ }
/* FP:lib.rs-0386 */ 
/* FP:lib.rs-0387 */ impl<'a> Id<'a> {
/* FP:lib.rs-0388 */     /// Creates an `Id` named `name`.
/* FP:lib.rs-0389 */     ///
/* FP:lib.rs-0390 */     /// The caller must ensure that the input conforms to an
/* FP:lib.rs-0391 */     /// identifier format: it must be a non-empty string made up of
/* FP:lib.rs-0392 */     /// alphanumeric or underscore characters, not beginning with a
/* FP:lib.rs-0393 */     /// digit (i.e., the regular expression `[a-zA-Z_][a-zA-Z_0-9]*`).
/* FP:lib.rs-0394 */     ///
/* FP:lib.rs-0395 */     /// (Note: this format is a strict subset of the `ID` format
/* FP:lib.rs-0396 */     /// defined by the DOT language. This function may change in the
/* FP:lib.rs-0397 */     /// future to accept a broader subset, or the entirety, of DOT's
/* FP:lib.rs-0398 */     /// `ID` format.)
/* FP:lib.rs-0399 */     ///
/* FP:lib.rs-0400 */     /// Passing an invalid string (containing spaces, brackets,
/* FP:lib.rs-0401 */     /// quotes, ...) will return an empty `Err` value.
/* FP:lib.rs-0402 */     pub fn new<Name: Into<Cow<'a, str>>>(name: Name) -> Result<Id<'a>, ()> {
/* FP:lib.rs-0403 */         let name = name.into();
/* FP:lib.rs-0404 */         match name.chars().next() {
/* FP:lib.rs-0405 */             Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
/* FP:lib.rs-0406 */             _ => return Err(()),
/* FP:lib.rs-0407 */         }
/* FP:lib.rs-0408 */         if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
/* FP:lib.rs-0409 */             return Err(());
/* FP:lib.rs-0410 */         }
/* FP:lib.rs-0411 */ 
/* FP:lib.rs-0412 */         Ok(Id { name })
/* FP:lib.rs-0413 */     }
/* FP:lib.rs-0414 */ 
/* FP:lib.rs-0415 */     pub fn as_slice(&'a self) -> &'a str {
/* FP:lib.rs-0416 */         &self.name
/* FP:lib.rs-0417 */     }
/* FP:lib.rs-0418 */ }
/* FP:lib.rs-0419 */ 
/* FP:lib.rs-0420 */ /// Each instance of a type that implements `Label<C>` maps to a
/* FP:lib.rs-0421 */ /// unique identifier with respect to `C`, which is used to identify
/* FP:lib.rs-0422 */ /// it in the generated .dot file. They can also provide more
/* FP:lib.rs-0423 */ /// elaborate (and non-unique) label text that is used in the graphviz
/* FP:lib.rs-0424 */ /// rendered output.
/* FP:lib.rs-0425 */ 
/* FP:lib.rs-0426 */ /// The graph instance is responsible for providing the DOT compatible
/* FP:lib.rs-0427 */ /// identifiers for the nodes and (optionally) rendered labels for the nodes and
/* FP:lib.rs-0428 */ /// edges, as well as an identifier for the graph itself.
/* FP:lib.rs-0429 */ pub trait Labeller<'a> {
/* FP:lib.rs-0430 */     type Node;
/* FP:lib.rs-0431 */     type Edge;
/* FP:lib.rs-0432 */ 
/* FP:lib.rs-0433 */     /// Must return a DOT compatible identifier naming the graph.
/* FP:lib.rs-0434 */     fn graph_id(&'a self) -> Id<'a>;
/* FP:lib.rs-0435 */ 
/* FP:lib.rs-0436 */     /// Maps `n` to a unique identifier with respect to `self`. The
/* FP:lib.rs-0437 */     /// implementor is responsible for ensuring that the returned name
/* FP:lib.rs-0438 */     /// is a valid DOT identifier.
/* FP:lib.rs-0439 */     fn node_id(&'a self, n: &Self::Node) -> Id<'a>;
/* FP:lib.rs-0440 */ 
/* FP:lib.rs-0441 */     /// Maps `n` to one of the [graphviz `shape` names][1]. If `None`
/* FP:lib.rs-0442 */     /// is returned, no `shape` attribute is specified.
/* FP:lib.rs-0443 */     ///
/* FP:lib.rs-0444 */     /// [1]: https://www.graphviz.org/doc/info/shapes.html
/* FP:lib.rs-0445 */     fn node_shape(&'a self, _node: &Self::Node) -> Option<LabelText<'a>> {
/* FP:lib.rs-0446 */         None
/* FP:lib.rs-0447 */     }
/* FP:lib.rs-0448 */ 
/* FP:lib.rs-0449 */     /// Maps `n` to a label that will be used in the rendered output.
/* FP:lib.rs-0450 */     /// The label need not be unique, and may be the empty string; the
/* FP:lib.rs-0451 */     /// default is just the output from `node_id`.
/* FP:lib.rs-0452 */     fn node_label(&'a self, n: &Self::Node) -> LabelText<'a> {
/* FP:lib.rs-0453 */         LabelStr(self.node_id(n).name)
/* FP:lib.rs-0454 */     }
/* FP:lib.rs-0455 */ 
/* FP:lib.rs-0456 */     /// Maps `e` to a label that will be used in the rendered output.
/* FP:lib.rs-0457 */     /// The label need not be unique, and may be the empty string; the
/* FP:lib.rs-0458 */     /// default is in fact the empty string.
/* FP:lib.rs-0459 */     fn edge_label(&'a self, _e: &Self::Edge) -> LabelText<'a> {
/* FP:lib.rs-0460 */         LabelStr("".into())
/* FP:lib.rs-0461 */     }
/* FP:lib.rs-0462 */ 
/* FP:lib.rs-0463 */     /// Maps `n` to a style that will be used in the rendered output.
/* FP:lib.rs-0464 */     fn node_style(&'a self, _n: &Self::Node) -> Style {
/* FP:lib.rs-0465 */         Style::None
/* FP:lib.rs-0466 */     }
/* FP:lib.rs-0467 */ 
/* FP:lib.rs-0468 */     /// Maps `e` to a style that will be used in the rendered output.
/* FP:lib.rs-0469 */     fn edge_style(&'a self, _e: &Self::Edge) -> Style {
/* FP:lib.rs-0470 */         Style::None
/* FP:lib.rs-0471 */     }
/* FP:lib.rs-0472 */ }
/* FP:lib.rs-0473 */ 
/* FP:lib.rs-0474 */ /// Escape tags in such a way that it is suitable for inclusion in a
/* FP:lib.rs-0475 */ /// Graphviz HTML label.
/* FP:lib.rs-0476 */ pub fn escape_html(s: &str) -> String {
/* FP:lib.rs-0477 */     s.replace('&', "&amp;")
/* FP:lib.rs-0478 */         .replace('\"', "&quot;")
/* FP:lib.rs-0479 */         .replace('<', "&lt;")
/* FP:lib.rs-0480 */         .replace('>', "&gt;")
/* FP:lib.rs-0481 */         .replace('\n', "<br align=\"left\"/>")
/* FP:lib.rs-0482 */ }
/* FP:lib.rs-0483 */ 
/* FP:lib.rs-0484 */ impl<'a> LabelText<'a> {
/* FP:lib.rs-0485 */     pub fn label<S: Into<Cow<'a, str>>>(s: S) -> LabelText<'a> {
/* FP:lib.rs-0486 */         LabelStr(s.into())
/* FP:lib.rs-0487 */     }
/* FP:lib.rs-0488 */ 
/* FP:lib.rs-0489 */     pub fn html<S: Into<Cow<'a, str>>>(s: S) -> LabelText<'a> {
/* FP:lib.rs-0490 */         HtmlStr(s.into())
/* FP:lib.rs-0491 */     }
/* FP:lib.rs-0492 */ 
/* FP:lib.rs-0493 */     fn escape_char<F>(c: char, mut f: F)
/* FP:lib.rs-0494 */     where
/* FP:lib.rs-0495 */         F: FnMut(char),
/* FP:lib.rs-0496 */     {
/* FP:lib.rs-0497 */         match c {
/* FP:lib.rs-0498 */             // not escaping \\, since Graphviz escString needs to
/* FP:lib.rs-0499 */             // interpret backslashes; see EscStr above.
/* FP:lib.rs-0500 */             '\\' => f(c),
/* FP:lib.rs-0501 */             _ => {
/* FP:lib.rs-0502 */                 for c in c.escape_default() {
/* FP:lib.rs-0503 */                     f(c)
/* FP:lib.rs-0504 */                 }
/* FP:lib.rs-0505 */             }
/* FP:lib.rs-0506 */         }
/* FP:lib.rs-0507 */     }
/* FP:lib.rs-0508 */     fn escape_str(s: &str) -> String {
/* FP:lib.rs-0509 */         let mut out = String::with_capacity(s.len());
/* FP:lib.rs-0510 */         for c in s.chars() {
/* FP:lib.rs-0511 */             LabelText::escape_char(c, |c| out.push(c));
/* FP:lib.rs-0512 */         }
/* FP:lib.rs-0513 */         out
/* FP:lib.rs-0514 */     }
/* FP:lib.rs-0515 */ 
/* FP:lib.rs-0516 */     /// Renders text as string suitable for a label in a .dot file.
/* FP:lib.rs-0517 */     /// This includes quotes or suitable delimiters.
/* FP:lib.rs-0518 */     pub fn to_dot_string(&self) -> String {
/* FP:lib.rs-0519 */         match *self {
/* FP:lib.rs-0520 */             LabelStr(ref s) => format!("\"{}\"", s.escape_default()),
/* FP:lib.rs-0521 */             EscStr(ref s) => format!("\"{}\"", LabelText::escape_str(s)),
/* FP:lib.rs-0522 */             HtmlStr(ref s) => format!("<{s}>"),
/* FP:lib.rs-0523 */         }
/* FP:lib.rs-0524 */     }
/* FP:lib.rs-0525 */ }
/* FP:lib.rs-0526 */ 
/* FP:lib.rs-0527 */ pub type Nodes<'a, N> = Cow<'a, [N]>;
/* FP:lib.rs-0528 */ pub type Edges<'a, E> = Cow<'a, [E]>;
/* FP:lib.rs-0529 */ 
/* FP:lib.rs-0530 */ // (The type parameters in GraphWalk should be associated items,
/* FP:lib.rs-0531 */ // when/if Rust supports such.)
/* FP:lib.rs-0532 */ 
/* FP:lib.rs-0533 */ /// GraphWalk is an abstraction over a directed graph = (nodes,edges)
/* FP:lib.rs-0534 */ /// made up of node handles `N` and edge handles `E`, where each `E`
/* FP:lib.rs-0535 */ /// can be mapped to its source and target nodes.
/* FP:lib.rs-0536 */ ///
/* FP:lib.rs-0537 */ /// The lifetime parameter `'a` is exposed in this trait (rather than
/* FP:lib.rs-0538 */ /// introduced as a generic parameter on each method declaration) so
/* FP:lib.rs-0539 */ /// that a client impl can choose `N` and `E` that have substructure
/* FP:lib.rs-0540 */ /// that is bound by the self lifetime `'a`.
/* FP:lib.rs-0541 */ ///
/* FP:lib.rs-0542 */ /// The `nodes` and `edges` method each return instantiations of
/* FP:lib.rs-0543 */ /// `Cow<[T]>` to leave implementors the freedom to create
/* FP:lib.rs-0544 */ /// entirely new vectors or to pass back slices into internally owned
/* FP:lib.rs-0545 */ /// vectors.
/* FP:lib.rs-0546 */ pub trait GraphWalk<'a> {
/* FP:lib.rs-0547 */     type Node: Clone;
/* FP:lib.rs-0548 */     type Edge: Clone;
/* FP:lib.rs-0549 */ 
/* FP:lib.rs-0550 */     /// Returns all the nodes in this graph.
/* FP:lib.rs-0551 */     fn nodes(&'a self) -> Nodes<'a, Self::Node>;
/* FP:lib.rs-0552 */     /// Returns all of the edges in this graph.
/* FP:lib.rs-0553 */     fn edges(&'a self) -> Edges<'a, Self::Edge>;
/* FP:lib.rs-0554 */     /// The source node for `edge`.
/* FP:lib.rs-0555 */     fn source(&'a self, edge: &Self::Edge) -> Self::Node;
/* FP:lib.rs-0556 */     /// The target node for `edge`.
/* FP:lib.rs-0557 */     fn target(&'a self, edge: &Self::Edge) -> Self::Node;
/* FP:lib.rs-0558 */ }
/* FP:lib.rs-0559 */ 
/* FP:lib.rs-0560 */ #[derive(Clone, PartialEq, Eq, Debug)]
/* FP:lib.rs-0561 */ pub enum RenderOption {
/* FP:lib.rs-0562 */     NoEdgeLabels,
/* FP:lib.rs-0563 */     NoNodeLabels,
/* FP:lib.rs-0564 */     NoEdgeStyles,
/* FP:lib.rs-0565 */     NoNodeStyles,
/* FP:lib.rs-0566 */ 
/* FP:lib.rs-0567 */     Fontname(String),
/* FP:lib.rs-0568 */     DarkTheme,
/* FP:lib.rs-0569 */ }
/* FP:lib.rs-0570 */ 
/* FP:lib.rs-0571 */ /// Renders directed graph `g` into the writer `w` in DOT syntax.
/* FP:lib.rs-0572 */ /// (Simple wrapper around `render_opts` that passes a default set of options.)
/* FP:lib.rs-0573 */ pub fn render<'a, N, E, G, W>(g: &'a G, w: &mut W) -> io::Result<()>
/* FP:lib.rs-0574 */ where
/* FP:lib.rs-0575 */     N: Clone + 'a,
/* FP:lib.rs-0576 */     E: Clone + 'a,
/* FP:lib.rs-0577 */     G: Labeller<'a, Node = N, Edge = E> + GraphWalk<'a, Node = N, Edge = E>,
/* FP:lib.rs-0578 */     W: Write,
/* FP:lib.rs-0579 */ {
/* FP:lib.rs-0580 */     render_opts(g, w, &[])
/* FP:lib.rs-0581 */ }
/* FP:lib.rs-0582 */ 
/* FP:lib.rs-0583 */ /// Renders directed graph `g` into the writer `w` in DOT syntax.
/* FP:lib.rs-0584 */ /// (Main entry point for the library.)
/* FP:lib.rs-0585 */ pub fn render_opts<'a, N, E, G, W>(g: &'a G, w: &mut W, options: &[RenderOption]) -> io::Result<()>
/* FP:lib.rs-0586 */ where
/* FP:lib.rs-0587 */     N: Clone + 'a,
/* FP:lib.rs-0588 */     E: Clone + 'a,
/* FP:lib.rs-0589 */     G: Labeller<'a, Node = N, Edge = E> + GraphWalk<'a, Node = N, Edge = E>,
/* FP:lib.rs-0590 */     W: Write,
/* FP:lib.rs-0591 */ {
/* FP:lib.rs-0592 */     writeln!(w, "digraph {} {{", g.graph_id().as_slice())?;
/* FP:lib.rs-0593 */ 
/* FP:lib.rs-0594 */     // Global graph properties
/* FP:lib.rs-0595 */     let mut graph_attrs = Vec::new();
/* FP:lib.rs-0596 */     let mut content_attrs = Vec::new();
/* FP:lib.rs-0597 */     let font;
/* FP:lib.rs-0598 */     if let Some(fontname) = options.iter().find_map(|option| {
/* FP:lib.rs-0599 */         if let RenderOption::Fontname(fontname) = option { Some(fontname) } else { None }
/* FP:lib.rs-0600 */     }) {
/* FP:lib.rs-0601 */         font = format!(r#"fontname="{fontname}""#);
/* FP:lib.rs-0602 */         graph_attrs.push(&font[..]);
/* FP:lib.rs-0603 */         content_attrs.push(&font[..]);
/* FP:lib.rs-0604 */     }
/* FP:lib.rs-0605 */     if options.contains(&RenderOption::DarkTheme) {
/* FP:lib.rs-0606 */         graph_attrs.push(r#"bgcolor="black""#);
/* FP:lib.rs-0607 */         graph_attrs.push(r#"fontcolor="white""#);
/* FP:lib.rs-0608 */         content_attrs.push(r#"color="white""#);
/* FP:lib.rs-0609 */         content_attrs.push(r#"fontcolor="white""#);
/* FP:lib.rs-0610 */     }
/* FP:lib.rs-0611 */     if !(graph_attrs.is_empty() && content_attrs.is_empty()) {
/* FP:lib.rs-0612 */         writeln!(w, r#"    graph[{}];"#, graph_attrs.join(" "))?;
/* FP:lib.rs-0613 */         let content_attrs_str = content_attrs.join(" ");
/* FP:lib.rs-0614 */         writeln!(w, r#"    node[{content_attrs_str}];"#)?;
/* FP:lib.rs-0615 */         writeln!(w, r#"    edge[{content_attrs_str}];"#)?;
/* FP:lib.rs-0616 */     }
/* FP:lib.rs-0617 */ 
/* FP:lib.rs-0618 */     let mut text = Vec::new();
/* FP:lib.rs-0619 */     for n in g.nodes().iter() {
/* FP:lib.rs-0620 */         write!(w, "    ")?;
/* FP:lib.rs-0621 */         let id = g.node_id(n);
/* FP:lib.rs-0622 */ 
/* FP:lib.rs-0623 */         let escaped = &g.node_label(n).to_dot_string();
/* FP:lib.rs-0624 */ 
/* FP:lib.rs-0625 */         write!(text, "{}", id.as_slice()).unwrap();
/* FP:lib.rs-0626 */ 
/* FP:lib.rs-0627 */         if !options.contains(&RenderOption::NoNodeLabels) {
/* FP:lib.rs-0628 */             write!(text, "[label={escaped}]").unwrap();
/* FP:lib.rs-0629 */         }
/* FP:lib.rs-0630 */ 
/* FP:lib.rs-0631 */         let style = g.node_style(n);
/* FP:lib.rs-0632 */         if !options.contains(&RenderOption::NoNodeStyles) && style != Style::None {
/* FP:lib.rs-0633 */             write!(text, "[style=\"{}\"]", style.as_slice()).unwrap();
/* FP:lib.rs-0634 */         }
/* FP:lib.rs-0635 */ 
/* FP:lib.rs-0636 */         if let Some(s) = g.node_shape(n) {
/* FP:lib.rs-0637 */             write!(text, "[shape={}]", &s.to_dot_string()).unwrap();
/* FP:lib.rs-0638 */         }
/* FP:lib.rs-0639 */ 
/* FP:lib.rs-0640 */         writeln!(text, ";").unwrap();
/* FP:lib.rs-0641 */         w.write_all(&text)?;
/* FP:lib.rs-0642 */ 
/* FP:lib.rs-0643 */         text.clear();
/* FP:lib.rs-0644 */     }
/* FP:lib.rs-0645 */ 
/* FP:lib.rs-0646 */     for e in g.edges().iter() {
/* FP:lib.rs-0647 */         let escaped_label = &g.edge_label(e).to_dot_string();
/* FP:lib.rs-0648 */         write!(w, "    ")?;
/* FP:lib.rs-0649 */         let source = g.source(e);
/* FP:lib.rs-0650 */         let target = g.target(e);
/* FP:lib.rs-0651 */         let source_id = g.node_id(&source);
/* FP:lib.rs-0652 */         let target_id = g.node_id(&target);
/* FP:lib.rs-0653 */ 
/* FP:lib.rs-0654 */         write!(text, "{} -> {}", source_id.as_slice(), target_id.as_slice()).unwrap();
/* FP:lib.rs-0655 */ 
/* FP:lib.rs-0656 */         if !options.contains(&RenderOption::NoEdgeLabels) {
/* FP:lib.rs-0657 */             write!(text, "[label={escaped_label}]").unwrap();
/* FP:lib.rs-0658 */         }
/* FP:lib.rs-0659 */ 
/* FP:lib.rs-0660 */         let style = g.edge_style(e);
/* FP:lib.rs-0661 */         if !options.contains(&RenderOption::NoEdgeStyles) && style != Style::None {
/* FP:lib.rs-0662 */             write!(text, "[style=\"{}\"]", style.as_slice()).unwrap();
/* FP:lib.rs-0663 */         }
/* FP:lib.rs-0664 */ 
/* FP:lib.rs-0665 */         writeln!(text, ";").unwrap();
/* FP:lib.rs-0666 */         w.write_all(&text)?;
/* FP:lib.rs-0667 */ 
/* FP:lib.rs-0668 */         text.clear();
/* FP:lib.rs-0669 */     }
/* FP:lib.rs-0670 */ 
/* FP:lib.rs-0671 */     writeln!(w, "}}")
/* FP:lib.rs-0672 */ }
/* FP:lib.rs-0673 */ 
/* FP:lib.rs-0674 */ #[cfg(test)]