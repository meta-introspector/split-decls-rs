macro_rules! deps {
    () => {
        Stack!();
        StackDelegate!();
        Platform!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [doc = " Entry points for attribute query"] impl Stack { # [doc = " Append the `relative` path to the root directory of the cache and efficiently create leading directories, while assuring that no"] # [doc = " symlinks are in that path."] # [doc = " Unless `mode` is known with `Some(gix_index::entry::Mode::DIR|COMMIT)`,"] # [doc = " then `relative` points to a directory itself in which case the entire resulting path is created as directory."] # [doc = " If it's not known it is assumed to be a file."] # [doc = " `objects` maybe used to lookup objects from an [id mapping][crate::stack::State::id_mappings_from_index()], with mappnigs"] # [doc = ""] # [doc = " Provide access to cached information for that `relative` path via the returned platform."] pub fn at_path (& mut self , relative : impl ToNormalPathComponents , mode : Option < gix_index :: entry :: Mode > , objects : & dyn gix_object :: Find ,) -> std :: io :: Result < Platform < '_ > > { self . statistics . platforms += 1 ; let mut delegate = StackDelegate { state : & mut self . state , buf : & mut self . buf , mode , id_mappings : & self . id_mappings , objects , case : self . case , statistics : & mut self . statistics , } ; self . stack . make_relative_path_current (relative , & mut delegate) ? ; Ok (Platform { parent : self , is_dir : mode_is_dir (mode) , }) } # [doc = " Obtain a platform for lookups from a repo-`relative` path, typically obtained from an index entry. `mode` should reflect"] # [doc = " the kind of item set here, or left at `None` if unknown."] # [doc = " `objects` maybe used to lookup objects from an [id mapping][crate::stack::State::id_mappings_from_index()]."] # [doc = " All effects are similar to [`at_path()`][Self::at_path()]."] # [doc = ""] # [doc = " If `relative` ends with `/` and `mode` is `None`, it is automatically assumed set to be a directory."] pub fn at_entry < 'r > (& mut self , relative : impl Into < & 'r BStr > , mode : Option < gix_index :: entry :: Mode > , objects : & dyn gix_object :: Find ,) -> std :: io :: Result < Platform < '_ > > { let relative = relative . into () ; self . at_path (relative , mode . or_else (| | relative . ends_with_str ("/") . then_some (gix_index :: entry :: Mode :: DIR)) , objects ,) } }
    };
}

impl_6!();