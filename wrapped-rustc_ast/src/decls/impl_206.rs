macro_rules! deps {
    () => {
        FieldDef!();
        VariantData!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl VariantData { # [doc = " Return the fields of this variant."] pub fn fields (& self) -> & [FieldDef] { match self { VariantData :: Struct { fields , .. } | VariantData :: Tuple (fields , _) => fields , _ => & [] , } } # [doc = " Return the `NodeId` of this variant's constructor, if it has one."] pub fn ctor_node_id (& self) -> Option < NodeId > { match * self { VariantData :: Struct { .. } => None , VariantData :: Tuple (_ , id) | VariantData :: Unit (id) => Some (id) , } } }
    };
}

impl_206!();