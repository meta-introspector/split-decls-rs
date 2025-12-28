macro_rules! deps {
    () => {
        ModuleId!();
        ModuleItemMap!();
        HasResolver!();
        Resolver!();
        DefDatabase!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl HasResolver for ModuleId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { let (mut def_map , local_def_map) = self . local_def_map (db) ; let mut module_id = self . local_id ; if ! self . is_within_block () { return Resolver { scopes : vec ! [] , module_scope : ModuleItemMap { def_map , local_def_map , module_id } , } ; } let mut modules : SmallVec < _ , 1 > = smallvec ! [] ; while let Some (parent) = def_map . parent () { let block_def_map = mem :: replace (& mut def_map , parent . def_map (db)) ; let block_module_id = mem :: replace (& mut module_id , parent . local_id) ; modules . push ((block_def_map , block_module_id)) ; if ! parent . is_within_block () { break ; } } let mut resolver = Resolver { scopes : Vec :: with_capacity (modules . len ()) , module_scope : ModuleItemMap { def_map , local_def_map , module_id } , } ; for (def_map , module_id) in modules . into_iter () . rev () { resolver = resolver . push_block_scope (def_map , local_def_map , module_id) ; } resolver } }
    };
}

impl_327!()