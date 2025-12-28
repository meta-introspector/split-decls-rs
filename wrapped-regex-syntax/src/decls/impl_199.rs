macro_rules! deps {
    () => {
        Flag!();
        Flags!();
        FlagsItemKind!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl Flags { fn from_ast (ast : & ast :: Flags) -> Flags { let mut flags = Flags :: default () ; let mut enable = true ; for item in & ast . items { match item . kind { ast :: FlagsItemKind :: Negation => { enable = false ; } ast :: FlagsItemKind :: Flag (ast :: Flag :: CaseInsensitive) => { flags . case_insensitive = Some (enable) ; } ast :: FlagsItemKind :: Flag (ast :: Flag :: MultiLine) => { flags . multi_line = Some (enable) ; } ast :: FlagsItemKind :: Flag (ast :: Flag :: DotMatchesNewLine) => { flags . dot_matches_new_line = Some (enable) ; } ast :: FlagsItemKind :: Flag (ast :: Flag :: SwapGreed) => { flags . swap_greed = Some (enable) ; } ast :: FlagsItemKind :: Flag (ast :: Flag :: Unicode) => { flags . unicode = Some (enable) ; } ast :: FlagsItemKind :: Flag (ast :: Flag :: CRLF) => { flags . crlf = Some (enable) ; } ast :: FlagsItemKind :: Flag (ast :: Flag :: IgnoreWhitespace) => { } } } flags } fn merge (& mut self , previous : & Flags) { if self . case_insensitive . is_none () { self . case_insensitive = previous . case_insensitive ; } if self . multi_line . is_none () { self . multi_line = previous . multi_line ; } if self . dot_matches_new_line . is_none () { self . dot_matches_new_line = previous . dot_matches_new_line ; } if self . swap_greed . is_none () { self . swap_greed = previous . swap_greed ; } if self . unicode . is_none () { self . unicode = previous . unicode ; } if self . crlf . is_none () { self . crlf = previous . crlf ; } } fn case_insensitive (& self) -> bool { self . case_insensitive . unwrap_or (false) } fn multi_line (& self) -> bool { self . multi_line . unwrap_or (false) } fn dot_matches_new_line (& self) -> bool { self . dot_matches_new_line . unwrap_or (false) } fn swap_greed (& self) -> bool { self . swap_greed . unwrap_or (false) } fn unicode (& self) -> bool { self . unicode . unwrap_or (true) } fn crlf (& self) -> bool { self . crlf . unwrap_or (false) } }
    };
}

impl_199!()