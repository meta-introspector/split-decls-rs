use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod tt {
    pub use span::Span;
    pub use tt::{token_to_literal, DelimiterKind, IdentIsRaw, LitKind, Spacing};
    pub type Delimiter = ::tt::Delimiter<Span>;
    pub type DelimSpan = ::tt::DelimSpan<Span>;
    pub type Subtree = ::tt::Subtree<Span>;
    pub type Leaf = ::tt::Leaf<Span>;
    pub type Literal = ::tt::Literal<Span>;
    pub type Punct = ::tt::Punct<Span>;
    pub type Ident = ::tt::Ident<Span>;
    pub type TokenTree = ::tt::TokenTree<Span>;
    pub type TopSubtree = ::tt::TopSubtree<Span>;
    pub type TopSubtreeBuilder = ::tt::TopSubtreeBuilder<Span>;
    pub type TokenTreesView<'a> = ::tt::TokenTreesView<'a, Span>;
    pub type SubtreeView<'a> = ::tt::SubtreeView<'a, Span>;
    pub type TtElement<'a> = ::tt::iter::TtElement<'a, Span>;
    pub type TtIter<'a> = ::tt::iter::TtIter<'a, Span>;
}
