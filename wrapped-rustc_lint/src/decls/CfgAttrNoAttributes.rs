macro_rules! CfgAttrNoAttributes {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_cfg_attr_no_attributes)] pub (crate) struct CfgAttrNoAttributes ;
    };
}

CfgAttrNoAttributes!();