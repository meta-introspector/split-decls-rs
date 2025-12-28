macro_rules! join_bstr_unix_pathsep {
    () => {
        # [doc = " Join `path` to `base` such that they are separated with a `/`, i.e. `base/path`."] pub fn join_bstr_unix_pathsep < 'a , 'b > (base : impl Into < Cow < 'a , BStr > > , path : impl Into < & 'b BStr >) -> Cow < 'a , BStr > { let mut base = base . into () ; if ! base . is_empty () && base . last () != Some (& b'/') { base . to_mut () . push (b'/') ; } base . to_mut () . extend_from_slice (path . into ()) ; base }
    };
}

join_bstr_unix_pathsep!()