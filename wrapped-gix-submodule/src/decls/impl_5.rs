macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [doc = " Mutation"] impl File { # [doc = " This can be used to let `config` override some values we know about submodules, namely…"] # [doc = ""] # [doc = " * `url`"] # [doc = " * `fetchRecurseSubmodules`"] # [doc = " * `ignore`"] # [doc = " * `update`"] # [doc = " * `branch`"] # [doc = ""] # [doc = " These values aren't validated yet, which will happen upon query."] pub fn append_submodule_overrides (& mut self , config : & gix_config :: File < '_ >) -> & mut Self { let mut values = BTreeMap :: < _ , Vec < _ > > :: new () ; for (module_name , section) in config . sections_by_name ("submodule") . into_iter () . flatten () . filter_map (| s | s . header () . subsection_name () . map (| n | (n , s))) { for field in ["url" , "fetchRecurseSubmodules" , "ignore" , "update" , "branch"] { if let Some (value) = section . value (field) { values . entry ((module_name , field)) . or_default () . push (value) ; } } } let values = { let mut v : Vec < _ > = values . into_iter () . collect () ; v . sort_by_key (| a | a . 0 . 0) ; v } ; let mut config_to_append = gix_config :: File :: new (config . meta_owned ()) ; let mut prev_name = None ; for ((module_name , field) , values) in values { if prev_name != Some (module_name) { config_to_append . new_section ("submodule" , Some (Cow :: Owned (module_name . to_owned ()))) . expect ("all names come from valid configuration, so remain valid") ; prev_name = Some (module_name) ; } config_to_append . section_mut ("submodule" , Some (module_name)) . expect ("always set at this point") . push (field . try_into () . expect ("statically known key") , Some (values . last () . expect ("at least one value or we wouldn't be here")) ,) ; } self . config . append (config_to_append) ; self } }
    };
}

impl_5!()