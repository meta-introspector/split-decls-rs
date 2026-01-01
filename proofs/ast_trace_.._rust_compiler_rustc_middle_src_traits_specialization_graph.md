# AST Trace: ../rust/compiler/rustc_middle/src/traits/specialization_graph.rs

Generated 16 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use rustc_data_structures::fx::FxIndexMap;
use rustc_errors::ErrorGuaranteed;
use rustc_hir::def_id::{DefId, DefIdMap};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_macros::{HashStable, TyDecodable, TyEncodable};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use rustc_span::sym;

use crate::error::StrictCoherenceNeedsNegativeCoherence;
use crate::ty::fast_reject::SimplifiedType;
use crate::ty::{self, TyCtxt, TypeVisitableExt};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=Graph | COMPLEXITY=8 | LINES=25

```rust
/// A per-trait graph of impls in specialization order. At the moment, this
/// graph forms a tree rooted with the trait itself, with all other nodes
/// representing impls, and parent-child relationships representing
/// specializations.
///
/// The graph provides two key services:
///
/// - Construction. This implicitly checks for overlapping impls (i.e., impls
///   that overlap but where neither specializes the other -- an artifact of the
///   simple "chain" rule.
///
/// - Parent extraction. In particular, the graph can give you the *immediate*
///   parents of a given specializing impl, which is needed for extracting
///   default items amongst other things. In the simple "chain" rule, every impl
///   has at most one parent.
#[derive(TyEncodable, TyDecodable, HashStable, Debug)]
pub struct Graph {
    /// All impls have a parent; the "root" impls have as their parent the `def_id`
    /// of the trait.
    pub parent: DefIdMap<DefId>,

    /// The "root" impls are found by looking up the trait's def_id.
    pub children: DefIdMap<Children>,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=new | COMPLEXITY=8 | LINES=13

```rust
impl Graph {
    pub fn new() -> Graph {
        Graph { parent: Default::default(), children: Default::default() }
    }

    /// The parent of a given impl, which is the `DefId` of the trait when the
    /// impl is a "specialization root".
    #[track_caller]
    pub fn parent(&self, child: DefId) -> DefId {
        *self.parent.get(&child).unwrap_or_else(|| panic!("Failed to get parent for {child:?}"))
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=13 | LINES=12

```rust
/// What kind of overlap check are we doing -- this exists just for testing and feature-gating
/// purposes.
#[derive(Copy, Clone, PartialEq, Eq, Hash, HashStable, Debug, TyEncodable, TyDecodable)]
pub enum OverlapMode {
    /// The 1.0 rules (either types fail to unify, or where clauses are not implemented for crate-local types)
    Stable,
    /// Feature-gated test: Stable, *or* there is an explicit negative impl that rules out one of the where-clauses.
    WithNegative,
    /// Just check for negative impls, not for "where clause not implemented": used for testing.
    Strict,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=get | COMPLEXITY=20 | LINES=35

```rust
impl OverlapMode {
    pub fn get(tcx: TyCtxt<'_>, trait_id: DefId) -> OverlapMode {
        let with_negative_coherence = tcx.features().with_negative_coherence();
        let strict_coherence = tcx.has_attr(trait_id, sym::rustc_strict_coherence);

        if with_negative_coherence {
            if strict_coherence { OverlapMode::Strict } else { OverlapMode::WithNegative }
        } else {
            if strict_coherence {
                let attr_span = trait_id
                    .as_local()
                    .into_iter()
                    .flat_map(|local_def_id| {
                        tcx.hir_attrs(tcx.local_def_id_to_hir_id(local_def_id))
                    })
                    .find(|attr| attr.has_name(sym::rustc_strict_coherence))
                    .map(|attr| attr.span());
                tcx.dcx().emit_err(StrictCoherenceNeedsNegativeCoherence {
                    span: tcx.def_span(trait_id),
                    attr_span,
                });
            }
            OverlapMode::Stable
        }
    }

    pub fn use_negative_impl(&self) -> bool {
        *self == OverlapMode::Strict || *self == OverlapMode::WithNegative
    }

    pub fn use_implicit_negative(&self) -> bool {
        *self == OverlapMode::Stable || *self == OverlapMode::WithNegative
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=Children | COMPLEXITY=7 | LINES=20

```rust
/// Children of a given impl, grouped into blanket/non-blanket varieties as is
/// done in `TraitDef`.
#[derive(Default, TyEncodable, TyDecodable, Debug, HashStable)]
pub struct Children {
    // Impls of a trait (or specializations of a given impl). To allow for
    // quicker lookup, the impls are indexed by a simplified version of their
    // `Self` type: impls with a simplifiable `Self` are stored in
    // `non_blanket_impls` keyed by it, while all other impls are stored in
    // `blanket_impls`.
    //
    // A similar division is used within `TraitDef`, but the lists there collect
    // together *all* the impls for a trait, and are populated prior to building
    // the specialization graph.
    /// Impls of the trait.
    pub non_blanket_impls: FxIndexMap<SimplifiedType, Vec<DefId>>,

    /// Blanket impls associated with the trait.
    pub blanket_impls: Vec<DefId>,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
/// A node in the specialization graph is either an impl or a trait
/// definition; either can serve as a source of item definitions.
/// There is always exactly one trait definition node: the root.
#[derive(Debug, Copy, Clone)]
pub enum Node {
    Impl(DefId),
    Trait(DefId),
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=is_from_trait | COMPLEXITY=15 | LINES=28

```rust
impl Node {
    pub fn is_from_trait(&self) -> bool {
        matches!(self, Node::Trait(..))
    }

    /// Tries to find the associated item that implements `trait_item_def_id`
    /// defined in this node.
    ///
    /// If this returns `None`, the item can potentially still be found in
    /// parents of this node.
    pub fn item<'tcx>(&self, tcx: TyCtxt<'tcx>, trait_item_def_id: DefId) -> Option<ty::AssocItem> {
        match *self {
            Node::Trait(_) => Some(tcx.associated_item(trait_item_def_id)),
            Node::Impl(impl_def_id) => {
                let id = tcx.impl_item_implementor_ids(impl_def_id).get(&trait_item_def_id)?;
                Some(tcx.associated_item(*id))
            }
        }
    }

    pub fn def_id(&self) -> DefId {
        match *self {
            Node::Impl(did) => did,
            Node::Trait(did) => did,
        }
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=Ancestors | COMPLEXITY=2 | LINES=7

```rust
#[derive(Copy, Clone)]
pub struct Ancestors<'tcx> {
    trait_def_id: DefId,
    specialization_graph: &'tcx Graph,
    current_source: Option<Node>,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=next | COMPLEXITY=12 | LINES=17

```rust
impl Iterator for Ancestors<'_> {
    type Item = Node;
    fn next(&mut self) -> Option<Node> {
        let cur = self.current_source.take();
        if let Some(Node::Impl(cur_impl)) = cur {
            let parent = self.specialization_graph.parent(cur_impl);

            self.current_source = if parent == self.trait_def_id {
                Some(Node::Trait(parent))
            } else {
                Some(Node::Impl(parent))
            };
        }
        cur
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=LeafDef | COMPLEXITY=16 | LINES=36

```rust
/// Information about the most specialized definition of an associated item.
#[derive(Debug)]
pub struct LeafDef {
    /// The associated item described by this `LeafDef`.
    pub item: ty::AssocItem,

    /// The node in the specialization graph containing the definition of `item`.
    pub defining_node: Node,

    /// The "top-most" (ie. least specialized) specialization graph node that finalized the
    /// definition of `item`.
    ///
    /// Example:
    ///
    /// ```
    /// #![feature(specialization)]
    /// trait Tr {
    ///     fn assoc(&self);
    /// }
    ///
    /// impl<T> Tr for T {
    ///     default fn assoc(&self) {}
    /// }
    ///
    /// impl Tr for u8 {}
    /// ```
    ///
    /// If we start the leaf definition search at `impl Tr for u8`, that impl will be the
    /// `finalizing_node`, while `defining_node` will be the generic impl.
    ///
    /// If the leaf definition search is started at the generic impl, `finalizing_node` will be
    /// `None`, since the most specialized impl we found still allows overriding the method
    /// (doesn't finalize it).
    pub finalizing_node: Option<Node>,
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=is_final | COMPLEXITY=3 | LINES=7

```rust
impl LeafDef {
    /// Returns whether this definition is known to not be further specializable.
    pub fn is_final(&self) -> bool {
        self.finalizing_node.is_some()
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=leaf_def | COMPLEXITY=16 | LINES=27

```rust
impl<'tcx> Ancestors<'tcx> {
    /// Finds the bottom-most (ie. most specialized) definition of an associated
    /// item.
    pub fn leaf_def(mut self, tcx: TyCtxt<'tcx>, trait_item_def_id: DefId) -> Option<LeafDef> {
        let mut finalizing_node = None;

        self.find_map(|node| {
            if let Some(item) = node.item(tcx, trait_item_def_id) {
                if finalizing_node.is_none() {
                    let is_specializable = item.defaultness(tcx).is_default()
                        || tcx.defaultness(node.def_id()).is_default();

                    if !is_specializable {
                        finalizing_node = Some(node);
                    }
                }

                Some(LeafDef { item, defining_node: node, finalizing_node })
            } else {
                // Item not mentioned. This "finalizes" any defaulted item provided by an ancestor.
                finalizing_node = Some(node);
                None
            }
        })
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=ancestors | COMPLEXITY=12 | LINES=23

```rust
/// Walk up the specialization ancestors of a given impl, starting with that
/// impl itself.
///
/// Returns `Err` if an error was reported while building the specialization
/// graph.
pub fn ancestors(
    tcx: TyCtxt<'_>,
    trait_def_id: DefId,
    start_from_impl: DefId,
) -> Result<Ancestors<'_>, ErrorGuaranteed> {
    let specialization_graph = tcx.specialization_graph_of(trait_def_id)?;

    if let Err(reported) = tcx.type_of(start_from_impl).instantiate_identity().error_reported() {
        Err(reported)
    } else {
        Ok(Ancestors {
            trait_def_id,
            specialization_graph,
            current_source: Some(Node::Impl(start_from_impl)),
        })
    }
}
```

---
*Generated by AST tracing system*
