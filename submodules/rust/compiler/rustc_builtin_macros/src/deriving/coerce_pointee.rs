mkuse!{use ast :: HasAttrs ;}
mkuse!{use rustc_ast :: mut_visit :: MutVisitor ;}
mkuse!{use rustc_ast :: visit :: BoundKind ;}
mkuse!{use rustc_ast :: { self as ast , GenericArg , GenericBound , GenericParamKind , Generics , ItemKind , MetaItem , TraitBoundModifiers , VariantData , WherePredicate , } ;}
mkuse!{use rustc_data_structures :: flat_map_in_place :: FlatMapInPlace ;}
mkuse!{use rustc_errors :: E0802 ;}
mkuse!{use rustc_expand :: base :: { Annotatable , ExtCtxt } ;}
mkuse!{use rustc_macros :: Diagnostic ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol , sym } ;}
mkuse!{use thin_vec :: { ThinVec , thin_vec } ;}
mkuse!{use crate :: errors ;}
mkitem!{macro_rules ! path { ($ span : expr , $ ($ part : ident) ::*) => { vec ! [$ (Ident :: new (sym ::$ part , $ span) ,) *] } }}

macro_rules! expand_deriving_coerce_pointee_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_deriving_coerce_pointee in module {}", module_path!());
    };
}

mkfn!{
    expand_deriving_coerce_pointee_introspect!();
    pub (crate) fn expand_deriving_coerce_pointee (cx : & ExtCtxt < '_ > , span : Span , _mitem : & MetaItem , item : & Annotatable , push : & mut dyn FnMut (Annotatable) , _is_const : bool ,) { item . visit_with (& mut DetectNonGenericPointeeAttr { cx }) ; let (name_ident , generics) = if let Annotatable :: Item (aitem) = item && let ItemKind :: Struct (ident , g , struct_data) = & aitem . kind { if ! matches ! (struct_data , VariantData :: Struct { fields , recovered : _ } | VariantData :: Tuple (fields , _) if ! fields . is_empty ()) { cx . dcx () . emit_err (RequireOneField { span }) ; return ; } (* ident , g) } else { cx . dcx () . emit_err (RequireTransparent { span }) ; return ; } ; let self_params : Vec < _ > = generics . params . iter () . map (| p | match p . kind { GenericParamKind :: Lifetime => GenericArg :: Lifetime (cx . lifetime (p . span () , p . ident)) , GenericParamKind :: Type { .. } => GenericArg :: Type (cx . ty_ident (p . span () , p . ident)) , GenericParamKind :: Const { .. } => GenericArg :: Const (cx . const_ident (p . span () , p . ident)) , }) . collect () ; let type_params : Vec < _ > = generics . params . iter () . enumerate () . filter_map (| (idx , p) | { if let GenericParamKind :: Type { .. } = p . kind { Some ((idx , p . span () , p . attrs () . iter () . any (| attr | attr . has_name (sym :: pointee)))) } else { None } }) . collect () ; let pointee_param_idx = if type_params . is_empty () { cx . dcx () . emit_err (RequireOneGeneric { span }) ; return ; } else if type_params . len () == 1 { type_params [0] . 0 } else { let mut pointees = type_params . iter () . filter_map (| & (idx , span , is_pointee) | is_pointee . then_some ((idx , span))) ; match (pointees . next () , pointees . next ()) { (Some ((idx , _span)) , None) => idx , (None , _) => { cx . dcx () . emit_err (RequireOnePointee { span }) ; return ; } (Some ((_ , one)) , Some ((_ , another))) => { cx . dcx () . emit_err (TooManyPointees { one , another }) ; return ; } } } ; let path = cx . path_all (span , false , vec ! [name_ident] , self_params . clone ()) ; let self_type = cx . ty_path (path) ; let attrs = thin_vec ! [cx . attr_word (sym :: automatically_derived , span) ,] ; { let trait_path = cx . path_all (span , true , path ! (span , core :: marker :: CoercePointeeValidated) , vec ! []) ; let trait_ref = cx . trait_ref (trait_path) ; push (Annotatable :: Item (cx . item (span , attrs . clone () , ast :: ItemKind :: Impl (ast :: Impl { generics : Generics { params : generics . params . iter () . map (| p | match & p . kind { GenericParamKind :: Lifetime => { cx . lifetime_param (p . span () , p . ident , p . bounds . clone ()) } GenericParamKind :: Type { default : _ } => { cx . typaram (p . span () , p . ident , p . bounds . clone () , None) } GenericParamKind :: Const { ty , span : _ , default : _ } => cx . const_param (p . span () , p . ident , p . bounds . clone () , ty . clone () , None ,) , }) . collect () , where_clause : generics . where_clause . clone () , span : generics . span , } , of_trait : Some (Box :: new (ast :: TraitImplHeader { safety : ast :: Safety :: Default , polarity : ast :: ImplPolarity :: Positive , defaultness : ast :: Defaultness :: Final , constness : ast :: Const :: No , trait_ref , })) , self_ty : self_type . clone () , items : ThinVec :: new () , }) ,) ,)) ; } let mut add_impl_block = | generics , trait_symbol , trait_args | { let mut parts = path ! (span , core :: ops) ; parts . push (Ident :: new (trait_symbol , span)) ; let trait_path = cx . path_all (span , true , parts , trait_args) ; let trait_ref = cx . trait_ref (trait_path) ; let item = cx . item (span , attrs . clone () , ast :: ItemKind :: Impl (ast :: Impl { generics , of_trait : Some (Box :: new (ast :: TraitImplHeader { safety : ast :: Safety :: Default , polarity : ast :: ImplPolarity :: Positive , defaultness : ast :: Defaultness :: Final , constness : ast :: Const :: No , trait_ref , })) , self_ty : self_type . clone () , items : ThinVec :: new () , }) ,) ; push (Annotatable :: Item (item)) ; } ; let s_ty = cx . ty_ident (span , Ident :: new (sym :: __S , span)) ; let mut alt_self_params = self_params ; alt_self_params [pointee_param_idx] = GenericArg :: Type (s_ty . clone ()) ; let alt_self_type = cx . ty_path (cx . path_all (span , false , vec ! [name_ident] , alt_self_params)) ; let mut impl_generics = generics . clone () ; let pointee_ty_ident = generics . params [pointee_param_idx] . ident ; let mut self_bounds ; { let pointee = & mut impl_generics . params [pointee_param_idx] ; self_bounds = pointee . bounds . clone () ; if ! contains_maybe_sized_bound (& self_bounds) && ! contains_maybe_sized_bound_on_pointee (& generics . where_clause . predicates , pointee_ty_ident . name ,) { cx . dcx () . emit_err (RequiresMaybeSized { span : pointee_ty_ident . span , name : pointee_ty_ident , }) ; return ; } let arg = GenericArg :: Type (s_ty . clone ()) ; let unsize = cx . path_all (span , true , path ! (span , core :: marker :: Unsize) , vec ! [arg]) ; pointee . bounds . push (cx . trait_bound (unsize , false)) ; pointee . attrs . retain (| attr | ! attr . has_name (sym :: pointee)) ; } for (idx , (params , orig_params)) in impl_generics . params . iter_mut () . zip (& generics . params) . enumerate () { match & mut params . kind { ast :: GenericParamKind :: Const { default , .. } => * default = None , ast :: GenericParamKind :: Type { default } => * default = None , ast :: GenericParamKind :: Lifetime => { } } if idx != pointee_param_idx { for bound in & orig_params . bounds { let mut bound = bound . clone () ; let mut substitution = TypeSubstitution { from_name : pointee_ty_ident . name , to_ty : & s_ty , rewritten : false , } ; substitution . visit_param_bound (& mut bound , BoundKind :: Bound) ; if substitution . rewritten { params . bounds . push (bound) ; } } } } { let mut substitution = TypeSubstitution { from_name : pointee_ty_ident . name , to_ty : & s_ty , rewritten : false } ; for bound in & mut self_bounds { substitution . visit_param_bound (bound , BoundKind :: Bound) ; } } for predicate in & generics . where_clause . predicates { if let ast :: WherePredicateKind :: BoundPredicate (bound) = & predicate . kind { let mut substitution = TypeSubstitution { from_name : pointee_ty_ident . name , to_ty : & s_ty , rewritten : false , } ; let mut kind = ast :: WherePredicateKind :: BoundPredicate (bound . clone ()) ; substitution . visit_where_predicate_kind (& mut kind) ; if substitution . rewritten { let predicate = ast :: WherePredicate { attrs : predicate . attrs . clone () , kind , span : predicate . span , id : ast :: DUMMY_NODE_ID , is_placeholder : false , } ; impl_generics . where_clause . predicates . push (predicate) ; } } } let extra_param = cx . typaram (span , Ident :: new (sym :: __S , span) , self_bounds , None) ; impl_generics . params . insert (pointee_param_idx + 1 , extra_param) ; let gen_args = vec ! [GenericArg :: Type (alt_self_type)] ; add_impl_block (impl_generics . clone () , sym :: DispatchFromDyn , gen_args . clone ()) ; add_impl_block (impl_generics . clone () , sym :: CoerceUnsized , gen_args) ; }
}

macro_rules! contains_maybe_sized_bound_on_pointee_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function contains_maybe_sized_bound_on_pointee in module {}", module_path!());
    };
}

mkfn!{
    contains_maybe_sized_bound_on_pointee_introspect!();
    fn contains_maybe_sized_bound_on_pointee (predicates : & [WherePredicate] , pointee : Symbol) -> bool { for bound in predicates { if let ast :: WherePredicateKind :: BoundPredicate (bound) = & bound . kind && bound . bounded_ty . kind . is_simple_path () . is_some_and (| name | name == pointee) { for bound in & bound . bounds { if is_maybe_sized_bound (bound) { return true ; } } } } false }
}

macro_rules! is_maybe_sized_bound_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_maybe_sized_bound in module {}", module_path!());
    };
}

mkfn!{
    is_maybe_sized_bound_introspect!();
    fn is_maybe_sized_bound (bound : & GenericBound) -> bool { if let GenericBound :: Trait (trait_ref) = bound && let TraitBoundModifiers { polarity : ast :: BoundPolarity :: Maybe (_) , .. } = trait_ref . modifiers && is_sized_marker (& trait_ref . trait_ref . path) { true } else { false } }
}

macro_rules! contains_maybe_sized_bound_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function contains_maybe_sized_bound in module {}", module_path!());
    };
}

mkfn!{
    contains_maybe_sized_bound_introspect!();
    fn contains_maybe_sized_bound (bounds : & [GenericBound]) -> bool { bounds . iter () . any (is_maybe_sized_bound) }
}

macro_rules! path_segment_is_exact_match_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_segment_is_exact_match in module {}", module_path!());
    };
}

mkfn!{
    path_segment_is_exact_match_introspect!();
    fn path_segment_is_exact_match (path_segments : & [ast :: PathSegment] , syms : & [Symbol]) -> bool { path_segments . iter () . zip (syms) . all (| (segment , & symbol) | segment . ident . name == symbol) }
}

macro_rules! is_sized_marker_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_sized_marker in module {}", module_path!());
    };
}

mkfn!{
    is_sized_marker_introspect!();
    fn is_sized_marker (path : & ast :: Path) -> bool { const CORE_UNSIZE : [Symbol ; 3] = [sym :: core , sym :: marker , sym :: Sized] ; const STD_UNSIZE : [Symbol ; 3] = [sym :: std , sym :: marker , sym :: Sized] ; if path . segments . len () == 4 && path . is_global () { path_segment_is_exact_match (& path . segments [1 ..] , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments [1 ..] , & STD_UNSIZE) } else if path . segments . len () == 3 { path_segment_is_exact_match (& path . segments , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments , & STD_UNSIZE) } else { * path == sym :: Sized } }
}
mkitem!{mkstruct!{struct TypeSubstitution < 'a > { from_name : Symbol , to_ty : & 'a ast :: Ty , rewritten : bool , }}}
mkitem!{mkimpl!{impl < 'a > ast :: mut_visit :: MutVisitor for TypeSubstitution < 'a > { fn visit_ty (& mut self , ty : & mut ast :: Ty) { if let Some (name) = ty . kind . is_simple_path () && name == self . from_name { * ty = self . to_ty . clone () ; self . rewritten = true ; } else { ast :: mut_visit :: walk_ty (self , ty) ; } } fn visit_where_predicate_kind (& mut self , kind : & mut ast :: WherePredicateKind) { match kind { rustc_ast :: WherePredicateKind :: BoundPredicate (bound) => { bound . bound_generic_params . flat_map_in_place (| param | self . flat_map_generic_param (param)) ; self . visit_ty (& mut bound . bounded_ty) ; for bound in & mut bound . bounds { self . visit_param_bound (bound , BoundKind :: Bound) } } rustc_ast :: WherePredicateKind :: RegionPredicate (_) | rustc_ast :: WherePredicateKind :: EqPredicate (_) => { } } } }}}
mkitem!{mkstruct!{struct DetectNonGenericPointeeAttr < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }}}
mkitem!{mkimpl!{impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for DetectNonGenericPointeeAttr < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } fn visit_generic_param (& mut self , param : & 'a rustc_ast :: GenericParam) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; match & param . kind { GenericParamKind :: Type { default } => { rustc_ast :: visit :: visit_opt ! (error_on_pointee , visit_ty , default) ; } GenericParamKind :: Const { .. } | GenericParamKind :: Lifetime => { rustc_ast :: visit :: walk_generic_param (& mut error_on_pointee , param) ; } } } fn visit_ty (& mut self , t : & 'a rustc_ast :: Ty) -> Self :: Result { let mut error_on_pointee = AlwaysErrorOnGenericParam { cx : self . cx } ; error_on_pointee . visit_ty (t) } }}}
mkitem!{mkstruct!{struct AlwaysErrorOnGenericParam < 'a , 'b > { cx : & 'a ExtCtxt < 'b > , }}}
mkitem!{mkimpl!{impl < 'a , 'b > rustc_ast :: visit :: Visitor < 'a > for AlwaysErrorOnGenericParam < 'a , 'b > { fn visit_attribute (& mut self , attr : & 'a rustc_ast :: Attribute) -> Self :: Result { if attr . has_name (sym :: pointee) { self . cx . dcx () . emit_err (errors :: NonGenericPointee { span : attr . span }) ; } } }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_transparent , code = E0802)] struct RequireTransparent { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_field , code = E0802)] struct RequireOneField { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_generic , code = E0802)] struct RequireOneGeneric { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_one_pointee , code = E0802)] struct RequireOnePointee { # [primary_span] span : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_too_many_pointees , code = E0802)] struct TooManyPointees { # [primary_span] one : Span , # [label] another : Span , }}}
mkitem!{mkstruct!{# [derive (Diagnostic)] # [diag (builtin_macros_coerce_pointee_requires_maybe_sized , code = E0802)] struct RequiresMaybeSized { # [primary_span] span : Span , name : Ident , }}}