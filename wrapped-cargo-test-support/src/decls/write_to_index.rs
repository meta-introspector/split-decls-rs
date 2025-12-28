macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! write_to_index {
    () => {
        deps!();
        pub (crate) fn write_to_index (registry_path : & Path , name : & str , line : String , local : bool) { let file = cargo_util :: registry :: make_dep_path (name , false) ; let dst = if local { registry_path . join ("index") . join (& file) } else { registry_path . join (& file) } ; let prev = fs :: read_to_string (& dst) . unwrap_or_default () ; t ! (fs :: create_dir_all (dst . parent () . unwrap ())) ; t ! (fs :: write (& dst , prev + & line [..] + "\n")) ; if ! local { let repo = t ! (git2 :: Repository :: open (& registry_path)) ; let mut index = t ! (repo . index ()) ; t ! (index . add_path (Path :: new (& file))) ; t ! (index . write ()) ; let id = t ! (index . write_tree ()) ; let tree = t ! (repo . find_tree (id)) ; let sig = t ! (repo . signature ()) ; let parent = t ! (repo . refname_to_id ("refs/heads/master")) ; let parent = t ! (repo . find_commit (parent)) ; t ! (repo . commit (Some ("HEAD") , & sig , & sig , "Another commit" , & tree , & [& parent])) ; } }
    };
}

write_to_index!()