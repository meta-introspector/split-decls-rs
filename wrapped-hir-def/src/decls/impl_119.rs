macro_rules! deps {
    () => {
        LocalFieldId!();
        VariantFields!();
        FieldData!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl VariantFields { pub fn len (& self) -> usize { self . fields . len () } pub fn fields (& self) -> & Arena < FieldData > { & self . fields } pub fn field (& self , name : & Name) -> Option < LocalFieldId > { self . fields () . iter () . find_map (| (id , data) | if & data . name == name { Some (id) } else { None }) } }
    };
}

impl_119!()