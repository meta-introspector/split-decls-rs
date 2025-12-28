macro_rules! PtrCastAddAutoToObject {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_ptr_cast_add_auto_to_object , code = E0804)] # [note] # [help] pub (crate) struct PtrCastAddAutoToObject { # [primary_span] # [label] pub span : Span , pub traits_len : usize , pub traits : DiagSymbolList < String > , }
    };
}

PtrCastAddAutoToObject!()