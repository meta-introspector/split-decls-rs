macro_rules! deps {
    () => {
        Case!();
    };
}

macro_rules! strip_base_handle_recompute_basename_pos {
    () => {
        deps!();
        # [doc = "  Return`relative_path` as being relative to `base` along with an updated `basename_pos` if it was set."] # [doc = " `case` is respected for the comparison."] # [doc = ""] # [doc = " This is useful to turn repository-relative paths into paths relative to a particular search base."] pub fn strip_base_handle_recompute_basename_pos < 'a > (base : & BStr , relative_path : & 'a BStr , basename_pos : Option < usize > , case : Case ,) -> Option < (& 'a BStr , Option < usize >) > { Some ((match case { Case :: Sensitive => relative_path . strip_prefix (base . as_bytes ()) ? . as_bstr () , Case :: Fold => { let rela_dir = relative_path . get (.. base . len ()) ? ; if ! rela_dir . eq_ignore_ascii_case (base) { return None ; } & relative_path [base . len () ..] } } , basename_pos . and_then (| pos | { let pos = pos - base . len () ; (pos != 0) . then_some (pos) }) ,)) }
    };
}

strip_base_handle_recompute_basename_pos!()