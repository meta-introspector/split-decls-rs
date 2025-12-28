macro_rules! deps {
    () => {
        PathCompleter!();
        ValueCompleter!();
        CompletionCandidate!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl ValueCompleter for PathCompleter { fn complete (& self , current : & OsStr) -> Vec < CompletionCandidate > { let filter = self . filter . as_deref () . unwrap_or (& | _ | true) ; let mut current_dir_actual = None ; let current_dir = self . current_dir . as_deref () . or_else (| | { current_dir_actual = std :: env :: current_dir () . ok () ; current_dir_actual . as_deref () }) ; let mut candidates = complete_path (current , current_dir , filter) ; if self . stdio && current . is_empty () { candidates . push (CompletionCandidate :: new ("-") . help (Some ("stdio" . into ()))) ; } candidates } }
    };
}

impl_120!();