macro_rules! get_cfg_attrs {
    () => {
        pub fn get_cfg_attrs (attrs : & [Attribute]) -> Vec < Attribute > { attrs . iter () . filter (| attr | ! attr . path () . segments . is_empty () && attr . path () . segments [0] . ident == "cfg") . cloned () . collect () }
    };
}

get_cfg_attrs!()