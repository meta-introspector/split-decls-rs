macro_rules! deps {
    () => {
        DefUse!();
        LocalUseMapBuild!();
        Appearance!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl Visitor < '_ > for LocalUseMapBuild < '_ > { fn visit_local (& mut self , local : Local , context : PlaceContext , location : Location) { if self . locals_with_use_data [local] && let Some (def_use) = def_use :: categorize (context) { let first_appearance = match def_use { DefUse :: Def => & mut self . local_use_map . first_def_at [local] , DefUse :: Use => & mut self . local_use_map . first_use_at [local] , DefUse :: Drop => & mut self . local_use_map . first_drop_at [local] , } ; let point_index = self . location_map . point_from_location (location) ; let appearance_index = self . local_use_map . appearances . push (Appearance { point_index , next : * first_appearance }) ; * first_appearance = Some (appearance_index) ; } } }
    };
}

impl_447!();