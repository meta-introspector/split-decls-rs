macro_rules! default_track_diagnostic {
    () => {
        fn default_track_diagnostic < R > (diag : DiagInner , f : & mut dyn FnMut (DiagInner) -> R) -> R { (* f) (diag) }
    };
}

default_track_diagnostic!()