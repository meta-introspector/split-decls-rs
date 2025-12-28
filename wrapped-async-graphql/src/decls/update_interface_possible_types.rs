macro_rules! deps {
    () => {
        MetaType!();
        Object!();
        Interface!();
        Registry!();
    };
}

macro_rules! update_interface_possible_types {
    () => {
        deps!();
        fn update_interface_possible_types (types : & mut IndexMap < String , Type > , registry : & mut Registry) { let mut interfaces = registry . types . values_mut () . filter_map (| ty | match ty { MetaType :: Interface { name , possible_types , .. } => Some ((name , possible_types)) , _ => None , }) . collect :: < HashMap < _ , _ > > () ; let objs = types . values () . filter_map (| ty | match ty { Type :: Object (obj) => Some ((& obj . name , & obj . implements)) , _ => None , }) ; for (obj_name , implements) in objs { for interface in implements { if let Some (possible_types) = interfaces . get_mut (interface) { possible_types . insert (obj_name . clone ()) ; } } } }
    };
}

update_interface_possible_types!();