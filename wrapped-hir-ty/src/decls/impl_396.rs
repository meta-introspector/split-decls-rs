macro_rules! deps {
    () => {
        HirDatabase!();
        InherentImpls!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        impl InherentImpls { fn collect_def_map (db : & dyn HirDatabase , def_map : & DefMap) -> Self { let mut map = FxHashMap :: default () ; collect (db , def_map , & mut map) ; let mut map = map . into_iter () . map (| (self_ty , impls) | (self_ty , impls . into_boxed_slice ())) . collect :: < FxHashMap < _ , _ > > () ; map . shrink_to_fit () ; return Self { map } ; fn collect (db : & dyn HirDatabase , def_map : & DefMap , map : & mut FxHashMap < SimplifiedType , Vec < ImplId > > ,) { for (_module_id , module_data) in def_map . modules () { for impl_id in module_data . scope . impls () { let data = db . impl_signature (impl_id) ; if data . target_trait . is_some () { continue ; } let interner = DbInterner :: new_with (db , None , None) ; let self_ty = db . impl_self_ty (impl_id) ; let self_ty = self_ty . instantiate_identity () ; if let Some (self_ty) = simplify_type (interner , self_ty , TreatParams :: InstantiateWithInfer) { map . entry (self_ty) . or_default () . push (impl_id) ; } } for konst in module_data . scope . unnamed_consts () { let body = db . body (konst . into ()) ; for (_ , block_def_map) in body . blocks (db) { collect (db , block_def_map , map) ; } } } } } pub fn for_self_ty (& self , self_ty : & SimplifiedType) -> & [ImplId] { self . map . get (self_ty) . map (| it | & * * it) . unwrap_or_default () } pub fn for_each_crate_and_block (db : & dyn HirDatabase , krate : Crate , block : Option < BlockId > , for_each : & mut dyn FnMut (& InherentImpls) ,) { let blocks = std :: iter :: successors (block , | block | block . loc (db) . module . containing_block ()) ; blocks . filter_map (| block | Self :: for_block (db , block) . as_deref ()) . for_each (& mut * for_each) ; for_each (Self :: for_crate (db , krate)) ; } }
    };
}

impl_396!();