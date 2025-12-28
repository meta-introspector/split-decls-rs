macro_rules! deps {
    () => {
        EarlyContextAndPass!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < 'ecx , 'tcx , T : EarlyLintPass > EarlyContextAndPass < 'ecx , 'tcx , T > { # [allow (rustc :: diagnostic_outside_of_impl)] fn check_id (& mut self , id : ast :: NodeId) { for early_lint in self . context . buffered . take (id) { let BufferedEarlyLint { span , node_id : _ , lint_id , diagnostic } = early_lint ; self . context . opt_span_lint (lint_id . lint , span , | diag | match diagnostic { DecorateDiagCompat :: Builtin (b) => { diagnostics :: decorate_builtin_lint (self . context . sess () , self . tcx , b , diag) ; } DecorateDiagCompat :: Dynamic (d) => d . decorate_lint_box (diag) , }) ; } } # [doc = " Merge the lints specified by any lint attributes into the"] # [doc = " current lint context, call the provided function, then reset the"] # [doc = " lints in effect to their previous state."] fn with_lint_attrs < F > (& mut self , id : ast :: NodeId , attrs : & '_ [ast :: Attribute] , f : F) where F : FnOnce (& mut Self) , { let is_crate_node = id == ast :: CRATE_NODE_ID ; debug ! (? id) ; let push = self . context . builder . push (attrs , is_crate_node , None) ; debug ! ("early context: enter_attrs({:?})" , attrs) ; lint_callback ! (self , check_attributes , attrs) ; ensure_sufficient_stack (| | f (self)) ; debug ! ("early context: exit_attrs({:?})" , attrs) ; lint_callback ! (self , check_attributes_post , attrs) ; self . context . builder . pop (push) ; } }
    };
}

impl_173!()