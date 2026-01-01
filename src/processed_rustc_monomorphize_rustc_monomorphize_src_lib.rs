// SRC: ../rust/compiler/rustc_monomorphize/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=12 */
// tidy-alphabetical-start
#[feature(array_windows)]
#[feature(file_buffered)]
#[feature(if_let_guard)]
#[feature(impl_trait_in_assoc_type)]
#[feature(once_cell_get_mut)]
// tidy-alphabetical-end

use crate::rustc_complete::lang_items::LangItem;
use crate::rustc_complete::query::TyCtxtAt;
use crate::rustc_complete::ty::adjustment::CustomCoerceUnsized;
use crate::rustc_complete::ty::{self, Ty};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_complete::util::Providers;
use crate::rustc_complete::{bug, traits};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=9 */
use crate::rustc_complete::ErrorGuaranteed;


rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=custom_coerce_unsize_info | COMPLEXITY=12 | LINES=27 */

fn custom_coerce_unsize_info<'tcx>(
    tcx: TyCtxtAt<'tcx>,
    source_ty: Ty<'tcx>,
    target_ty: Ty<'tcx>,
) -> Result<CustomCoerceUnsized, ErrorGuaranteed> {
    let trait_ref = ty::TraitRef::new(
        tcx.tcx,
        tcx.require_lang_item(LangItem::CoerceUnsized, tcx.span),
        [source_ty, target_ty],
    );

    match tcx
        .codegen_select_candidate(ty::TypingEnv::fully_monomorphized().as_query_input(trait_ref))
    {
        Ok(traits::ImplSource::UserDefined(traits::ImplSourceUserDefinedData {
            impl_def_id,
            ..
        })) => Ok(tcx.coerce_unsized_info(impl_def_id)?.custom_kind.unwrap()),
        impl_source => {
            bug!(
                "invalid `CoerceUnsized` from {source_ty} to {target_ty}: impl_source: {:?}",
                impl_source
            );
        }
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=provide | COMPLEXITY=2 | LINES=5 */

pub fn provide(providers: &mut Providers) {
    partitioning::provide(providers);
    mono_checks::provide(providers);
}