macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! create_leading_directory {
    () => {
        deps!();
        # [cfg (feature = "attributes")] fn create_leading_directory (is_last_component : bool , stack : & gix_fs :: Stack , mode : Option < gix_index :: entry :: Mode > , mkdir_calls : & mut usize , unlink_on_collision : bool ,) -> std :: io :: Result < () > { if is_last_component && ! crate :: stack :: mode_is_dir (mode) . unwrap_or (false) { return Ok (()) ; } * mkdir_calls += 1 ; match std :: fs :: create_dir (stack . current ()) { Ok (()) => Ok (()) , Err (err) if err . kind () == std :: io :: ErrorKind :: AlreadyExists => { let meta = stack . current () . symlink_metadata () ? ; if meta . is_dir () { Ok (()) } else if unlink_on_collision { if meta . file_type () . is_symlink () { gix_fs :: symlink :: remove (stack . current ()) ? ; } else { std :: fs :: remove_file (stack . current ()) ? ; } * mkdir_calls += 1 ; std :: fs :: create_dir (stack . current ()) } else { Err (err) } } Err (err) => Err (err) , } }
    };
}

create_leading_directory!()