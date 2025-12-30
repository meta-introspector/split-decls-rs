// Generated macro for impl_7398 (impl)
macro_rules! Depcrate_misc_earlyimpl_7398 {
() => {
// Module: crate::misc_early
// Provides: {"impl_7398"}
// Dependencies: {}
impl MiscEarlyLints { fn check_lit (cx : & EarlyContext < '_ > , lit : token :: Lit , span : Span) { let lit_snip = match snippet_opt (cx , span) { Some (snip) if snip . starts_with (| c : char | c . is_ascii_digit ()) => snip , _ => return , } ; let lit_kind = LitKind :: from_token_lit (lit) ; if let Ok (LitKind :: Int (value , lit_int_type)) = lit_kind { let suffix = match lit_int_type { LitIntType :: Signed (ty) => ty . name_str () , LitIntType :: Unsigned (ty) => ty . name_str () , LitIntType :: Unsuffixed => "" , } ; literal_suffix :: check (cx , span , & lit_snip , suffix , "integer") ; if lit_snip . starts_with ("0x") { mixed_case_hex_literals :: check (cx , span , suffix , & lit_snip) ; } else if lit_snip . starts_with ("0b") || lit_snip . starts_with ("0o") { } else if value != 0 && lit_snip . starts_with ('0') { zero_prefixed_literal :: check (cx , span , & lit_snip) ; } } else if let Ok (LitKind :: Float (_ , LitFloatType :: Suffixed (float_ty))) = lit_kind { let suffix = float_ty . name_str () ; literal_suffix :: check (cx , span , & lit_snip , suffix , "float") ; } } }
};
}
