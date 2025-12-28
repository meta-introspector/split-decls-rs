macro_rules! deps {
    () => {
        InferenceContext!();
        MethodError!();
        MethodResolutionContext!();
        MethodCallee!();
    };
}

macro_rules! impl_690 {
    () => {
        deps!();
        impl < 'a , 'db > InferenceContext < 'a , 'db > { # [doc = " Performs method lookup. If lookup is successful, it will return the callee"] # [doc = " and store an appropriate adjustment for the self-expr. In some cases it may"] # [doc = " report an error (e.g., invoking the `drop` method)."] # [instrument (level = "debug" , skip (self))] pub (crate) fn lookup_method_including_private (& mut self , self_ty : Ty < 'db > , name : Name , generic_args : Option < & HirGenericArgs > , receiver : ExprId , call_expr : ExprId ,) -> Result < (MethodCallee < 'db > , bool) , MethodError < 'db > > { let (pick , is_visible) = match self . lookup_probe (name , self_ty) { Ok (it) => (it , true) , Err (MethodError :: PrivateMatch (it)) => { (it , false) } Err (err) => return Err (err) , } ; let result = self . confirm_method (& pick , self_ty , call_expr , generic_args) ; debug ! ("result = {:?}" , result) ; if result . illegal_sized_bound { } self . write_expr_adj (receiver , result . adjustments) ; self . write_method_resolution (call_expr , result . callee . def_id , result . callee . args) ; Ok ((result . callee , is_visible)) } # [instrument (level = "debug" , skip (self))] pub (crate) fn lookup_probe (& self , method_name : Name , self_ty : Ty < 'db > ,) -> probe :: PickResult < 'db > { self . with_method_resolution (| ctx | { let pick = ctx . probe_for_name (probe :: Mode :: MethodCall , method_name , self_ty) ? ; Ok (pick) }) } pub (crate) fn with_method_resolution < R > (& self , f : impl FnOnce (& MethodResolutionContext < '_ , 'db >) -> R ,) -> R { let traits_in_scope = self . get_traits_in_scope () ; let traits_in_scope = match & traits_in_scope { Either :: Left (it) => it , Either :: Right (it) => * it , } ; let ctx = MethodResolutionContext { infcx : & self . table . infer_ctxt , resolver : & self . resolver , env : & self . table . trait_env , traits_in_scope , edition : self . edition , unstable_features : & self . unstable_features , } ; f (& ctx) } }
    };
}

impl_690!()