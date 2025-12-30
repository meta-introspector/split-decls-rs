// Generated macro for gitconfig_home_contents (function)
macro_rules! Depcrate_gitignoregitconfig_home_contents {
() => {
// Module: crate::gitignore
// Provides: {"gitconfig_home_contents"}
// Dependencies: {}
# [doc = " Returns the file contents of git's global config file, if one exists, in"] # [doc = " the user's home directory."] fn gitconfig_home_contents () -> Option < Vec < u8 > > { let home = match home_dir () { None => return None , Some (home) => home , } ; let mut file = match File :: open (home . join (".gitconfig")) { Err (_) => return None , Ok (file) => BufReader :: new (file) , } ; let mut contents = vec ! [] ; file . read_to_end (& mut contents) . ok () . map (| _ | contents) }
};
}
