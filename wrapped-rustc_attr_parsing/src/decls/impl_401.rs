macro_rules! deps {
    () => {
        AllowedTargets!();
        Stage!();
        AttributeParser!();
        InvalidTarget!();
        AllowedResult!();
        AcceptContext!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl < 'sess , S : Stage > AttributeParser < 'sess , S > { pub (crate) fn check_target (allowed_targets : & AllowedTargets , target : Target , cx : & mut AcceptContext < '_ , 'sess , S > ,) { match allowed_targets . is_allowed (target) { AllowedResult :: Allowed => { } AllowedResult :: Warn => { let allowed_targets = allowed_targets . allowed_targets () ; let (applied , only) = allowed_targets_applied (allowed_targets , target , cx . features) ; let name = cx . attr_path . clone () ; let attr_span = cx . attr_span ; cx . emit_lint (AttributeLintKind :: InvalidTarget { name , target , only : if only { "only " } else { "" } , applied , } , attr_span ,) ; } AllowedResult :: Error => { let allowed_targets = allowed_targets . allowed_targets () ; let (applied , only) = allowed_targets_applied (allowed_targets , target , cx . features) ; let name = cx . attr_path . clone () ; cx . dcx () . emit_err (InvalidTarget { span : cx . attr_span . clone () , name , target : target . plural_name () , only : if only { "only " } else { "" } , applied : DiagArgValue :: StrListSepByAnd (applied . into_iter () . map (Cow :: Owned) . collect () ,) , }) ; } } } pub (crate) fn check_type (attribute_type : AttributeType , target : Target , cx : & mut AcceptContext < '_ , 'sess , S > ,) { let is_crate_root = S :: id_is_crate_root (cx . target_id) ; if is_crate_root { return ; } if attribute_type != AttributeType :: CrateLevel { return ; } let lint = AttributeLintKind :: InvalidStyle { name : cx . attr_path . clone () , is_used_as_inner : cx . attr_style == AttrStyle :: Inner , target , target_span : cx . target_span , } ; let attr_span = cx . attr_span ; cx . emit_lint (lint , attr_span) ; } }
    };
}

impl_401!()