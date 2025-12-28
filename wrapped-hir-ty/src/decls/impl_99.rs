macro_rules! deps {
    () => {
        HirDatabase!();
        UninhabitedFrom!();
        VisiblyUninhabited!();
        EarlyBinder!();
        TraitEnvironment!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 'a , 'db > UninhabitedFrom < 'a , 'db > { fn new (infcx : & 'a InferCtxt < 'db > , target_mod : ModuleId , env : Arc < TraitEnvironment < 'db > > ,) -> Self { Self { target_mod , recursive_ty : FxHashSet :: default () , max_depth : 500 , infcx , env } } # [inline] fn interner (& self) -> DbInterner < 'db > { self . infcx . interner } # [inline] fn db (& self) -> & 'db dyn HirDatabase { self . interner () . db } fn visit_adt (& mut self , adt : AdtId , subst : GenericArgs < 'db > ,) -> ControlFlow < VisiblyUninhabited > { match adt { AdtId :: UnionId (_) => CONTINUE_OPAQUELY_INHABITED , AdtId :: StructId (s) => self . visit_variant (s . into () , subst) , AdtId :: EnumId (e) => { let enum_data = e . enum_variants (self . db ()) ; for & (variant , _ , _) in enum_data . variants . iter () { let variant_inhabitedness = self . visit_variant (variant . into () , subst) ; match variant_inhabitedness { Break (VisiblyUninhabited) => () , Continue (()) => return CONTINUE_OPAQUELY_INHABITED , } } BREAK_VISIBLY_UNINHABITED } } } fn visit_variant (& mut self , variant : VariantId , subst : GenericArgs < 'db > ,) -> ControlFlow < VisiblyUninhabited > { let variant_data = variant . fields (self . db ()) ; let fields = variant_data . fields () ; if fields . is_empty () { return CONTINUE_OPAQUELY_INHABITED ; } let is_enum = matches ! (variant , VariantId :: EnumVariantId (..)) ; let field_tys = self . db () . field_types (variant) ; let field_vis = if is_enum { None } else { Some (self . db () . field_visibilities (variant)) } ; for (fid , _) in fields . iter () { self . visit_field (field_vis . as_ref () . map (| it | it [fid]) , & field_tys [fid] , subst) ? ; } CONTINUE_OPAQUELY_INHABITED } fn visit_field (& mut self , vis : Option < Visibility > , ty : & EarlyBinder < 'db , Ty < 'db > > , subst : GenericArgs < 'db > ,) -> ControlFlow < VisiblyUninhabited > { if vis . is_none_or (| it | it . is_visible_from (self . db () , self . target_mod)) { let ty = ty . instantiate (self . interner () , subst) ; ty . visit_with (self) } else { CONTINUE_OPAQUELY_INHABITED } } }
    };
}

impl_99!()