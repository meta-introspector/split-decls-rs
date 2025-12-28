macro_rules! FerrisIdentifier {
    () => {
        # [derive (Diagnostic)] # [diag (interface_ferris_identifier)] pub struct FerrisIdentifier { # [primary_span] pub spans : Vec < Span > , # [suggestion (code = "{ferris_fix}" , applicability = "maybe-incorrect")] pub first_span : Span , pub ferris_fix : & 'static str , }
    };
}

FerrisIdentifier!()