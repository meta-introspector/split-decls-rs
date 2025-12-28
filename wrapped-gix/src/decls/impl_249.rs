macro_rules! deps {
    () => {
        Item!();
        PathspecDetached!();
        AttributeStack!();
        Pathspec!();
        Note!();
        Error!();
        Repository!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'repo > Pathspec < 'repo > { # [doc = " Create a new instance by parsing `patterns` into [`Pathspecs`](Pattern) to make them usable for searches."] # [doc = " `make_attribute` may be called if one of the patterns has a `(attr:a)` element which requires attribute matching. It should"] # [doc = " be used to control where attributes are coming from."] # [doc = " If `inherit_ignore_case` is `true`, the pathspecs may have their ignore-case default overridden to be case-insensitive by default."] # [doc = " This only works towards turning ignore-case for pathspecs on, but won't ever turn that setting off if."] # [doc = " If `empty_patterns_match_prefix` is `true`, then even empty patterns will match only what's inside of the prefix. Otherwise"] # [doc = " they will match everything."] # [doc = ""] # [doc = " ### Deviation"] # [doc = ""] # [doc = " Pathspecs can declare to be case-insensitive as part of their elements, which is a setting that is now respected for attribute"] # [doc = " queries as well."] pub fn new (repo : & 'repo Repository , empty_patterns_match_prefix : bool , patterns : impl IntoIterator < Item = impl AsRef < BStr > > , inherit_ignore_case : bool , make_attributes : impl FnOnce () -> Result < gix_worktree :: Stack , Box < dyn std :: error :: Error + Send + Sync + 'static > > ,) -> Result < Self , init :: Error > { let defaults = repo . pathspec_defaults_inherit_ignore_case (inherit_ignore_case) ? ; let patterns = patterns . into_iter () . map (move | p | parse (p . as_ref () , defaults)) . collect :: < Result < Vec < _ > , _ > > () ? ; let needs_cache = patterns . iter () . any (| p | ! p . attributes . is_empty ()) ; let prefix = if patterns . is_empty () && ! empty_patterns_match_prefix { None } else { repo . prefix () ? } ; let search = Search :: from_specs (patterns , prefix , & gix_path :: realpath_opts (repo . workdir () . unwrap_or_else (| | repo . git_dir ()) , repo . options . current_dir_or_empty () , gix_path :: realpath :: MAX_SYMLINKS ,) ? ,) ? ; let cache = needs_cache . then (make_attributes) . transpose () ? ; gix_trace :: debug ! (longest_prefix = ? search . longest_common_directory () , prefix_dir = ? search . prefix_directory () , patterns = ? search . patterns () . map (gix_pathspec :: Pattern :: path) . collect ::< Vec < _ >> ()) ; Ok (Self { repo , search , stack : cache , }) } # [doc = " Turn ourselves into the functional parts for direct usage."] # [doc = " Note that the [`cache`](AttributeStack) is only set if one of the [`search` patterns](Search)"] # [doc = " is specifying attributes to match for."] pub fn into_parts (self) -> (Search , Option < AttributeStack < 'repo > >) { (self . search , self . stack . map (| stack | AttributeStack :: new (stack , self . repo)) ,) } # [doc = " Turn ourselves into an implementation that works without a repository instance and that is rather minimal."] pub fn detach (self) -> std :: io :: Result < PathspecDetached > { Ok (PathspecDetached { search : self . search , stack : self . stack , odb : self . repo . objects . clone () . into_arc () ? , }) } }
    };
}

impl_249!();