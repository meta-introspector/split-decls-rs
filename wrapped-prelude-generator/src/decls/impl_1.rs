macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl Args { # [doc = " Resolves all relative paths in `exclude_paths` to absolute paths based on `self.path`."] pub fn resolve_exclude_paths (& self) -> anyhow :: Result < Vec < PathBuf > > { let mut resolved_paths = Vec :: new () ; for p in & self . exclude_paths { let resolved_p = if p . is_relative () { if * p == PathBuf :: from (".") { self . path . canonicalize () . context ("Failed to canonicalize project root path") ? } else { self . path . join (p) . canonicalize () . context (format ! ("Failed to canonicalize exclude path: {:?}" , p)) ? } } else { p . canonicalize () . context (format ! ("Failed to canonicalize absolute exclude path: {:?}" , p)) ? } ; resolved_paths . push (resolved_p) ; } Ok (resolved_paths) } }
    };
}

impl_1!();