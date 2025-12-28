macro_rules! OverflowingBinHexSignBitSub {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_sign_bit_suggestion , code = "{lit_no_suffix}{uint_ty} as {int_ty}" , applicability = "maybe-incorrect")] pub (crate) struct OverflowingBinHexSignBitSub < 'a > { # [primary_span] pub span : Span , pub lit_no_suffix : & 'a str , pub negative_val : String , pub uint_ty : & 'a str , pub int_ty : & 'a str , }
    };
}

OverflowingBinHexSignBitSub!()