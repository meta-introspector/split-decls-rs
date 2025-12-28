macro_rules! deps {
    () => {
        TypeCollectorVisitor!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < 'ast > Visit < 'ast > for TypeCollectorVisitor { fn visit_path (& mut self , i : & 'ast syn :: Path) { self . collected_types . insert (quote :: quote ! (# i) . to_string ()) ; self . walk_path (i) ; } fn visit_type (& mut self , i : & 'ast syn :: Type) { self . collected_types . insert (quote :: quote ! (# i) . to_string ()) ; match i { syn :: Type :: Array (type_array) => { self . visit_type (& type_array . elem) ; self . walk_type (i) ; } syn :: Type :: BareFn (type_bare_fn) => { self . walk_bare_fn (type_bare_fn) ; self . walk_type (i) ; } syn :: Type :: Group (type_group) => { self . visit_type (& type_group . elem) ; self . walk_type (i) ; } syn :: Type :: ImplTrait (type_impl_trait) => { for bound in & type_impl_trait . bounds { self . walk_type_param_bound (bound) ; } self . walk_type (i) ; } syn :: Type :: Infer (_) => { self . walk_type (i) ; } syn :: Type :: Macro (type_macro) => { self . walk_macro (& type_macro . mac) ; self . walk_type (i) ; } syn :: Type :: Never (_) => { self . walk_type (i) ; } syn :: Type :: Paren (type_paren) => { self . visit_type (& type_paren . elem) ; self . walk_type (i) ; } syn :: Type :: Path (type_path) => { self . walk_type_path (type_path) ; self . walk_type (i) ; } syn :: Type :: Ptr (type_ptr) => { self . visit_type (& type_ptr . elem) ; self . walk_type (i) ; } syn :: Type :: Reference (type_reference) => { self . visit_type (& type_reference . elem) ; self . walk_type (i) ; } syn :: Type :: Slice (type_slice) => { self . visit_type (& type_slice . elem) ; self . walk_type (i) ; } syn :: Type :: TraitObject (type_trait_object) => { for bound in & type_trait_object . bounds { self . walk_type_param_bound (bound) ; } self . walk_type (i) ; } syn :: Type :: Tuple (type_tuple) => { for elem in & type_tuple . elems { self . visit_type (elem) ; } self . walk_type (i) ; } syn :: Type :: Verbatim (_) => { self . walk_type (i) ; } _ => { self . walk_type (i) ; } } } }
    };
}

impl_171!()