macro_rules! deps {
    () => {
        ToolModule!();
        Crate!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl ToolModule { pub (crate) fn by_name (db : & dyn HirDatabase , krate : Crate , name : & str) -> Option < Self > { let krate = krate . id ; let idx = crate_def_map (db , krate) . registered_tools () . iter () . position (| it | it . as_str () == name) ? as u32 ; Some (ToolModule { krate , idx }) } pub fn name (& self , db : & dyn HirDatabase) -> Name { Name :: new_symbol_root (crate_def_map (db , self . krate) . registered_tools () [self . idx as usize] . clone () ,) } pub fn krate (& self) -> Crate { Crate { id : self . krate } } }
    };
}

impl_359!()