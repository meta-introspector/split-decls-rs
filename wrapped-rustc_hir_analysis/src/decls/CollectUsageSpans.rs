macro_rules! CollectUsageSpans {
    () => {
        # [doc = " Collect usages of the `param_def_id` and `Res::SelfTyAlias` in the HIR."] # [doc = ""] # [doc = " This is used to report places where the user has used parameters in a"] # [doc = " non-variance-constraining way for better bivariance errors."] struct CollectUsageSpans < 'a > { spans : & 'a mut Vec < Span > , param_def_id : DefId , }
    };
}

CollectUsageSpans!();