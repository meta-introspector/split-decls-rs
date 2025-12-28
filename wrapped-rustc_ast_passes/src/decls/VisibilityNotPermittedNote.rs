macro_rules! VisibilityNotPermittedNote {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum VisibilityNotPermittedNote { # [note (ast_passes_enum_variant)] EnumVariant , # [note (ast_passes_trait_impl)] TraitImpl , # [note (ast_passes_individual_impl_items)] IndividualImplItems , # [note (ast_passes_individual_foreign_items)] IndividualForeignItems , }
    };
}

VisibilityNotPermittedNote!();