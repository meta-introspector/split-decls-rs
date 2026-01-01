// SRC: ../rust/compiler/rustc_builtin_macros/src/deriving/bounds.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::MetaItem;
use crate::rustc_expand::base::{Annotatable, ExtCtxt};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=28 */
use crate::rustc_complete::Span;

use crate::deriving::generic::*;
use crate::deriving::path_std;

pub(crate) fn expand_deriving_copy(
    cx: &ExtCtxt<'_>,
    span: Span,
    mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
    is_const: bool,
) {
    let trait_def = TraitDef {
        span,
        path: path_std!(marker::Copy),
        skip_path_as_bound: false,
        needs_copy_as_bound_if_packed: false,
        additional_bounds: Vec::new(),
        supports_unions: true,
        methods: Vec::new(),
        associated_types: Vec::new(),
        is_const,
        is_staged_api_crate: cx.ecfg.features.staged_api(),
    };

    trait_def.expand(cx, mitem, item, push);
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=39 */

pub(crate) fn expand_deriving_const_param_ty(
    cx: &ExtCtxt<'_>,
    span: Span,
    mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
    is_const: bool,
) {
    let trait_def = TraitDef {
        span,
        path: path_std!(marker::ConstParamTy_),
        skip_path_as_bound: false,
        needs_copy_as_bound_if_packed: false,
        additional_bounds: vec![ty::Ty::Path(path_std!(cmp::Eq))],
        supports_unions: false,
        methods: Vec::new(),
        associated_types: Vec::new(),
        is_const,
        is_staged_api_crate: cx.ecfg.features.staged_api(),
    };

    trait_def.expand(cx, mitem, item, push);

    let trait_def = TraitDef {
        span,
        path: path_std!(marker::UnsizedConstParamTy),
        skip_path_as_bound: false,
        needs_copy_as_bound_if_packed: false,
        additional_bounds: vec![ty::Ty::Path(path_std!(cmp::Eq))],
        supports_unions: false,
        methods: Vec::new(),
        associated_types: Vec::new(),
        is_const,
        is_staged_api_crate: cx.ecfg.features.staged_api(),
    };

    trait_def.expand(cx, mitem, item, push);
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=4 | LINES=24 */

pub(crate) fn expand_deriving_unsized_const_param_ty(
    cx: &ExtCtxt<'_>,
    span: Span,
    mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
    is_const: bool,
) {
    let trait_def = TraitDef {
        span,
        path: path_std!(marker::UnsizedConstParamTy),
        skip_path_as_bound: false,
        needs_copy_as_bound_if_packed: false,
        additional_bounds: vec![ty::Ty::Path(path_std!(cmp::Eq))],
        supports_unions: false,
        methods: Vec::new(),
        associated_types: Vec::new(),
        is_const,
        is_staged_api_crate: cx.ecfg.features.staged_api(),
    };

    trait_def.expand(cx, mitem, item, push);
}