macro_rules! deps {
    () => {
        DirEntry!();
        WalkError!();
        GlobWalker!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Iterator for GlobWalker { type Item = Result < DirEntry , WalkError > ; fn next (& mut self) -> Option < Self :: Item > { let mut skip_dir = false ; 'skipper : loop { if skip_dir { self . walker . skip_current_dir () ; } for entry in & mut self . walker { match entry { Ok (e) => { let is_dir = e . file_type () . is_dir () ; let file_type = if e . file_type () . is_dir () { Some (FileType :: DIR) } else if e . file_type () . is_file () { Some (FileType :: FILE) } else if e . file_type () . is_symlink () { Some (FileType :: SYMLINK) } else { None } ; let file_type_matches = match (self . file_type_filter . as_ref () , file_type) { (None , _) => true , (Some (_) , None) => false , (Some (filter) , Some (actual)) => filter . contains (actual) , } ; let path = e . path () . strip_prefix (self . ignore . path ()) . unwrap () ; if path . as_os_str () . is_empty () { continue 'skipper ; } match self . ignore . matched (path , is_dir) { Match :: Whitelist (_) if file_type_matches => return Some (Ok (e)) , Match :: Ignore (_) if is_dir => { skip_dir = true ; continue 'skipper ; } _ => { } } } Err (e) => { return Some (Err (e)) ; } } } break ; } None } }
    };
}

impl_11!()