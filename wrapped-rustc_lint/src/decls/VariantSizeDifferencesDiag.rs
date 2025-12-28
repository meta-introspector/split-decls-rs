macro_rules! VariantSizeDifferencesDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_variant_size_differences)] pub (crate) struct VariantSizeDifferencesDiag { pub largest : u64 , }
    };
}

VariantSizeDifferencesDiag!();