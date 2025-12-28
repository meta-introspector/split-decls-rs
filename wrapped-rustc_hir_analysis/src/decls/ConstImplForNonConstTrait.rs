macro_rules! ConstImplForNonConstTrait {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_const_impl_for_non_const_trait)] pub (crate) struct ConstImplForNonConstTrait { # [primary_span] # [label] pub trait_ref_span : Span , pub trait_name : String , # [suggestion (applicability = "machine-applicable" , code = "#[const_trait] " , style = "verbose")] pub local_trait_span : Option < Span > , pub suggestion_pre : & 'static str , # [note] pub marking : () , # [note (hir_analysis_adding)] pub adding : () , }
    };
}

ConstImplForNonConstTrait!();