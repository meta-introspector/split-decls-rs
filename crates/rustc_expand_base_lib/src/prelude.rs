pub use rustc_ast::{self as ast, NodeId, mut_visit::MutVisitor, visit::AssocCtxt, HasNodeId, HasAttrs, AstNodeWrapper, MetaItemInner, MetaItemKind, StmtKind, ExprKind};
pub use rustc_span::{Span, Symbol, sym, LocalExpnId};
pub use rustc_feature::Features;
pub use rustc_hir::limit::Limit;
pub use smallvec::SmallVec;
pub use rustc_session::Session;
pub use std::sync::Arc;
pub use std::slice;
pub use std::mem;
pub use rustc_span::edition::Edition;
pub use thin_vec::ThinVec;

pub use rustc_ast_pretty::pprust;
pub use rustc_attr_parsing::{validate_attr, AttributeParser, ShouldEmit, EvalConfigResult, Early};
pub use rustc_hir::Target;
pub use rustc_lint_defs::builtin::{UNUSED_DOC_COMMENTS, UNUSED_ATTRIBUTES};

pub use crate::add_semicolon::AddSemicolon;
pub use crate::annotatable::Annotatable;
pub use crate::ast_fragment::{AstFragment, AstFragmentKind};
pub use crate::ast_traits::{
    AttributeHooks, AstFragmentConverter, CfgFalseExpandable, CfgFalseReporterContext,
    FlattensOutputs, HasDeclaredIdents, MacroInvocationNode, WalkableAstNode,
};
pub use crate::base_expansion_context::{BaseExpansionContext, ExpansionConfig, ResolverExpand};
pub use crate::cfg_false_reporter::CfgFalseReporter;
pub use crate::config::{attr_into_trace, configure, is_cfg, parse_cfg_old};
pub use crate::dummy_ast_node::DummyAstNode;
pub use crate::expanded_nodes::expanded_stmt::ExpandedStmt;
pub use crate::invocation_collector_node::{
    ExpandedArm, ExpandedExprField, ExpandedGenericParam, ExpandedItem, ExpandedPat, ExpandedParam,
    ExpandedAssocItem, ExpandedPatField, ExpandedFieldDef, ExpandedForeignItem, ExpandedVariant,
    ExpandedWherePredicate, ExpandedCrate, ExpandedExpr, ExpandedTy,
    InvocationCollectorNode,
};
pub use crate::invocation_data::{DirOwnership, ExpansionData, Invocation, InvocationKind, ModuleData};



//pub use crate::ast_fragments_defs::parse_ast_fragment; // Original path
//pub use crate::ast_fragments_split::invocation_collector::InvocationCollector; // Original path
pub use crate::mac_result::{DummyResult, MacResult};
pub use crate::placeholders::{placeholder, PlaceholderExpander};
pub use crate::resolver_traits::{DeriveResolutionProvider, ImportResolver, OpaqueDeriveResolution};
pub use crate::syntax_extension_trait::{MacroKindTrait, SyntaxExtensionTrait};
pub use crate::strip_unconfigured::{KeepCfg, StripUnconfigured};
