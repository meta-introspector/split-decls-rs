macro_rules! deps {
    () => {
        PathAncestors!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < 'a > Iterator for PathAncestors < 'a > { type Item = & 'a Path ; fn next (& mut self) -> Option < & 'a Path > { if let Some (path) = self . current { self . current = path . parent () ; if let Some (ref stop_at) = self . stop_at { if path == stop_at { self . current = None ; } } Some (path) } else { None } } }
    };
}

impl_26!();