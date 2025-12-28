macro_rules! deps {
    () => {
        AllowSelfProjection!();
        HirDatabase!();
    };
}

macro_rules! contains_illegal_self_type_reference {
    () => {
        deps!();
        fn contains_illegal_self_type_reference < 'db , T : rustc_type_ir :: TypeVisitable < DbInterner < 'db > > > (db : & 'db dyn HirDatabase , trait_ : TraitId , t : & T , allow_self_projection : AllowSelfProjection ,) -> bool { struct IllegalSelfTypeVisitor < 'db > { db : & 'db dyn HirDatabase , trait_ : TraitId , super_traits : Option < SmallVec < [TraitId ; 4] > > , allow_self_projection : AllowSelfProjection , } impl < 'db > rustc_type_ir :: TypeVisitor < DbInterner < 'db > > for IllegalSelfTypeVisitor < 'db > { type Result = ControlFlow < () > ; fn visit_ty (& mut self , ty : < DbInterner < 'db > as rustc_type_ir :: Interner > :: Ty ,) -> Self :: Result { let interner = DbInterner :: new_with (self . db , None , None) ; match ty . kind () { rustc_type_ir :: TyKind :: Param (param) if param . index == 0 => ControlFlow :: Break (()) , rustc_type_ir :: TyKind :: Param (_) => ControlFlow :: Continue (()) , rustc_type_ir :: TyKind :: Alias (AliasTyKind :: Projection , proj) => match self . allow_self_projection { AllowSelfProjection :: Yes => { let trait_ = proj . trait_def_id (DbInterner :: new_with (self . db , None , None)) ; let trait_ = match trait_ { SolverDefId :: TraitId (id) => id , _ => unreachable ! () , } ; if self . super_traits . is_none () { self . super_traits = Some (elaborate :: supertrait_def_ids (interner , self . trait_ . into ()) . map (| super_trait | super_trait . 0) . collect () ,) } if self . super_traits . as_ref () . is_some_and (| s | s . contains (& trait_)) { ControlFlow :: Continue (()) } else { ty . super_visit_with (self) } } AllowSelfProjection :: No => ty . super_visit_with (self) , } , _ => ty . super_visit_with (self) , } } } let mut visitor = IllegalSelfTypeVisitor { db , trait_ , super_traits : None , allow_self_projection } ; t . visit_with (& mut visitor) . is_break () }
    };
}

contains_illegal_self_type_reference!()