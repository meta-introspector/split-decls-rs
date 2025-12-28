macro_rules! deps {
    () => {
        Error!();
        State!();
        Action!();
        Outcome!();
        Delegate!();
        Context!();
        Kind!();
        Status!();
        ForDeletionMode!();
        Options!();
    };
}

macro_rules! recursive {
    () => {
        deps!();
        # [doc = " ### Deviation"] # [doc = ""] # [doc = " Git mostly silently ignores IO errors and stops iterating seemingly quietly, while we error loudly."] # [allow (clippy :: too_many_arguments)] pub (super) fn recursive (may_collapse : bool , current : & mut PathBuf , current_bstr : & mut BString , current_info : classify :: Outcome , ctx : & mut Context < '_ > , opts : Options < '_ > , delegate : & mut dyn Delegate , out : & mut Outcome , state : & mut State ,) -> Result < (Action , bool) , Error > { if ctx . should_interrupt . is_some_and (| flag | flag . load (Ordering :: Relaxed)) { return Err (Error :: Interrupted) ; } out . read_dir_calls += 1 ; let entries = gix_fs :: read_dir (current , opts . precompose_unicode) . map_err (| err | Error :: ReadDir { path : current . to_owned () , source : err , }) ? ; let mut num_entries = 0 ; let mark = state . mark (may_collapse) ; let mut prevent_collapse = false ; for entry in entries { let entry = entry . map_err (| err | Error :: DirEntry { parent_directory : current . to_owned () , source : err , }) ? ; num_entries += 1 ; let prev_len = current_bstr . len () ; if prev_len != 0 { current_bstr . push (b'/') ; } let file_name = entry . file_name () ; current_bstr . extend_from_slice (gix_path :: try_os_str_into_bstr (Cow :: Borrowed (file_name . as_ref ())) . expect ("no illformed UTF-8") . as_ref () ,) ; current . push (file_name) ; let mut info = classify :: path (current , current_bstr , if prev_len == 0 { 0 } else { prev_len + 1 } , None , | | entry . file_type () . ok () . map (Into :: into) , opts , ctx ,) ? ; if can_recurse (current_bstr . as_bstr () , info , opts . for_deletion , false , delegate ,) { let subdir_may_collapse = state . may_collapse (current) ; let (action , subdir_prevent_collapse) = recursive (subdir_may_collapse , current , current_bstr , info , ctx , opts , delegate , out , state ,) ? ; prevent_collapse |= subdir_prevent_collapse ; if action != Action :: Continue { return Ok ((action , prevent_collapse)) ; } } else { if opts . for_deletion == Some (ForDeletionMode :: IgnoredDirectoriesCanHideNestedRepositories) && info . disk_kind == Some (entry :: Kind :: Directory) && matches ! (info . status , Status :: Ignored (_)) { info . disk_kind = classify :: maybe_upgrade_to_repository (info . disk_kind , true , false , current , ctx . current_dir , ctx . git_dir_realpath ,) ; } if ! state . held_for_directory_collapse (current_bstr . as_bstr () , info , & opts) { let action = emit_entry (Cow :: Borrowed (current_bstr . as_bstr ()) , info , None , opts , out , delegate) ; if action != Action :: Continue { return Ok ((action , prevent_collapse)) ; } } } current_bstr . truncate (prev_len) ; current . pop () ; } let res = mark . reduce_held_entries (num_entries , state , & mut prevent_collapse , current , current_bstr . as_bstr () , current_info , opts , out , ctx , delegate ,) ; Ok ((res , prevent_collapse)) }
    };
}

recursive!()