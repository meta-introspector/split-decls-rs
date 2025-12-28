macro_rules! SimdIntrinsicArgConst {
    () => {
        # [derive (Diagnostic)] # [diag (borrowck_simd_intrinsic_arg_const)] pub (crate) struct SimdIntrinsicArgConst { # [primary_span] pub span : Span , pub arg : usize , pub intrinsic : String , }
    };
}

SimdIntrinsicArgConst!()