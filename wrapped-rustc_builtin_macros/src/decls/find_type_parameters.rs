macro_rules! deps {
    () => {
        Path!();
        DeriveMacroCall!();
        Ty!();
        TypeParameter!();
    };
}

macro_rules! find_type_parameters {
    () => {
        deps!();
        # [doc = " This method helps to extract all the type parameters referenced from a"] # [doc = " type. For a type parameter `<T>`, it looks for either a `TyPath` that"] # [doc = " is not global and starts with `T`, or a `TyQPath`."] # [doc = " Also include bound generic params from the input type."] fn find_type_parameters (ty : & ast :: Ty , ty_param_names : & [Symbol] , cx : & ExtCtxt < '_ > ,) -> Vec < TypeParameter > { use rustc_ast :: visit ; struct Visitor < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , ty_param_names : & 'a [Symbol] , bound_generic_params_stack : ThinVec < ast :: GenericParam > , type_params : Vec < TypeParameter > , } impl < 'a , 'b > visit :: Visitor < 'a > for Visitor < 'a , 'b > { fn visit_ty (& mut self , ty : & 'a ast :: Ty) { let stack_len = self . bound_generic_params_stack . len () ; if let ast :: TyKind :: FnPtr (fn_ptr) = & ty . kind && ! fn_ptr . generic_params . is_empty () { self . bound_generic_params_stack . extend (fn_ptr . generic_params . iter () . cloned ()) ; } if let ast :: TyKind :: Path (_ , path) = & ty . kind && let Some (segment) = path . segments . first () && self . ty_param_names . contains (& segment . ident . name) { self . type_params . push (TypeParameter { bound_generic_params : self . bound_generic_params_stack . clone () , ty : Box :: new (ty . clone ()) , }) ; } visit :: walk_ty (self , ty) ; self . bound_generic_params_stack . truncate (stack_len) ; } fn visit_poly_trait_ref (& mut self , trait_ref : & 'a ast :: PolyTraitRef) { let stack_len = self . bound_generic_params_stack . len () ; self . bound_generic_params_stack . extend (trait_ref . bound_generic_params . iter () . cloned ()) ; visit :: walk_poly_trait_ref (self , trait_ref) ; self . bound_generic_params_stack . truncate (stack_len) ; } fn visit_mac_call (& mut self , mac : & ast :: MacCall) { self . cx . dcx () . emit_err (errors :: DeriveMacroCall { span : mac . span () }) ; } } let mut visitor = Visitor { cx , ty_param_names , bound_generic_params_stack : ThinVec :: new () , type_params : Vec :: new () , } ; visit :: Visitor :: visit_ty (& mut visitor , ty) ; visitor . type_params }
    };
}

find_type_parameters!()