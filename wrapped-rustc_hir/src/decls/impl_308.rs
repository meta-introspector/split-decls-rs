macro_rules! deps {
    () => {
        CtorKind!();
        FieldDef!();
        VariantData!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl < 'hir > VariantData < 'hir > { # [doc = " Return the fields of this variant."] pub fn fields (& self) -> & 'hir [FieldDef < 'hir >] { match * self { VariantData :: Struct { fields , .. } | VariantData :: Tuple (fields , ..) => fields , _ => & [] , } } pub fn ctor (& self) -> Option < (CtorKind , HirId , LocalDefId) > { match * self { VariantData :: Tuple (_ , hir_id , def_id) => Some ((CtorKind :: Fn , hir_id , def_id)) , VariantData :: Unit (hir_id , def_id) => Some ((CtorKind :: Const , hir_id , def_id)) , VariantData :: Struct { .. } => None , } } # [inline] pub fn ctor_kind (& self) -> Option < CtorKind > { self . ctor () . map (| (kind , ..) | kind) } # [doc = " Return the `HirId` of this variant's constructor, if it has one."] # [inline] pub fn ctor_hir_id (& self) -> Option < HirId > { self . ctor () . map (| (_ , hir_id , _) | hir_id) } # [doc = " Return the `LocalDefId` of this variant's constructor, if it has one."] # [inline] pub fn ctor_def_id (& self) -> Option < LocalDefId > { self . ctor () . map (| (.. , def_id) | def_id) } }
    };
}

impl_308!()