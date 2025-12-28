macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! collect_args_fields {
    () => {
        deps!();
        pub (crate) fn collect_args_fields < 'a > (item : & 'a Item , fields : & 'a FieldsNamed ,) -> Result < Vec < (& 'a Field , Item) > , syn :: Error > { fields . named . iter () . map (| field | { let item = Item :: from_args_field (field , item . casing () , item . env_casing ()) ? ; Ok ((field , item)) }) . collect () }
    };
}

collect_args_fields!()