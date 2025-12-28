macro_rules! deps {
    () => {
        Visitor!();
        Field!();
        Union!();
        Interface!();
        VisitorContext!();
        FieldsOnCorrectType!();
        MetaType!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'a > Visitor < 'a > for FieldsOnCorrectType { fn enter_field (& mut self , ctx : & mut VisitorContext < 'a > , field : & 'a Positioned < Field >) { if let Some (parent_type) = ctx . parent_type () { if let Some (registry :: MetaType :: Union { .. }) | Some (registry :: MetaType :: Interface { .. }) = ctx . parent_type () { if field . node . name . node == "__typename" { return ; } } if parent_type . fields () . and_then (| fields | fields . get (field . node . name . node . as_str ())) . is_none () && ! field . node . directives . iter () . any (| directive | directive . node . name . node == "ifdef") { ctx . report_error (vec ! [field . pos] , format ! ("Unknown field \"{}\" on type \"{}\".{}" , field . node . name , parent_type . name () , if ctx . registry . enable_suggestions { make_suggestion (" Did you mean" , parent_type . fields () . iter () . map (| fields | fields . keys ()) . flatten () . map (String :: as_str) , & field . node . name . node ,) . unwrap_or_default () } else { String :: new () }) ,) ; } } } }
    };
}

impl_189!();