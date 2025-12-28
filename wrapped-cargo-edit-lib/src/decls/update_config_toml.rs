macro_rules! deps {
    () => {
        GeneratedPatches!();
    };
}

macro_rules! update_config_toml {
    () => {
        deps!();
        # [cfg (feature = "toml_edit_enabled")] # [doc = " Reads existing .cargo/config.toml, updates patch sections, and writes back."] pub fn update_config_toml (config_toml_path : & Path , new_patches : & GeneratedPatches ,) -> anyhow :: Result < () > { let mut doc = if config_toml_path . exists () { let contents = fs :: read_to_string (config_toml_path) . context (format ! ("Failed to read existing config.toml: {:?}" , config_toml_path)) ? ; contents . parse :: < DocumentMut > () . context (format ! ("Failed to parse existing config.toml: {:?}" , config_toml_path)) ? } else { println ! ("Warning: {:?} not found. Creating a new one." , config_toml_path) ; DocumentMut :: new () } ; let patches_table = doc . entry ("patch") . or_insert (Item :: Table (Table :: new ())) . as_table_mut () . context ("Expected 'patch' to be a table") ? ; for (repo_url , entries) in new_patches { let repo_table = patches_table . entry (repo_url) . or_insert (Item :: Table (Table :: new ())) . as_table_mut () . context (format ! ("Expected patch section for {} to be a table" , repo_url)) ? ; for entry in entries { let mut crate_table = Table :: new () ; crate_table . insert ("path" , Item :: Value (Value :: String (toml_edit :: Formatted :: new (entry . path . to_str () . context ("Invalid path") ? . to_string () ,))) ,) ; repo_table . insert (& entry . crate_name , Item :: Table (crate_table)) ; } } fs :: write (config_toml_path , doc . to_string ()) . context (format ! ("Failed to write updated config.toml: {:?}" , config_toml_path)) ? ; println ! ("Updated {:?}" , config_toml_path) ; Ok (()) }
    };
}

update_config_toml!()