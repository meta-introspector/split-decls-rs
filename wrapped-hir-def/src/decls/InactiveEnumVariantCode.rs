macro_rules! InactiveEnumVariantCode {
    () => {
        # [derive (Debug , PartialEq , Eq)] pub struct InactiveEnumVariantCode { pub cfg : CfgExpr , pub opts : CfgOptions , pub ast_id : span :: FileAstId < ast :: Variant > , }
    };
}

InactiveEnumVariantCode!();