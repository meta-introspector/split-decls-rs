macro_rules! deps {
    () => {
        Statistics!();
        PathIdMapping!();
        State!();
    };
}

macro_rules! StackDelegate {
    () => {
        deps!();
        pub (crate) struct StackDelegate < 'a , 'find > { pub state : & 'a mut State , pub buf : & 'a mut Vec < u8 > , # [cfg_attr (not (feature = "attributes") , allow (dead_code))] pub mode : Option < gix_index :: entry :: Mode > , pub id_mappings : & 'a Vec < PathIdMapping > , pub objects : & 'find dyn gix_object :: Find , pub case : gix_glob :: pattern :: Case , pub statistics : & 'a mut super :: Statistics , }
    };
}

StackDelegate!()