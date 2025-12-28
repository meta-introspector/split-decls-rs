macro_rules! deps {
    () => {
        StrLitKind!();
    };
}

macro_rules! format_patch {
    () => {
        deps!();
        fn format_patch (desired_indent : Option < usize > , patch : & str) -> String { let lit_kind = lit_kind_for_patch (patch) ; let indent = desired_indent . map (| it | " " . repeat (it)) ; let is_multiline = patch . contains ('\n') ; let mut buf = String :: new () ; if matches ! (lit_kind , StrLitKind :: Raw (_)) { buf . push ('[') ; } lit_kind . write_start (& mut buf) . unwrap () ; if is_multiline { buf . push ('\n') ; } let mut final_newline = false ; for line in lines_with_ends (patch) { if is_multiline && ! line . trim () . is_empty () { if let Some (indent) = & indent { buf . push_str (indent) ; buf . push_str ("    ") ; } } buf . push_str (line) ; final_newline = line . ends_with ('\n') ; } if final_newline { if let Some (indent) = & indent { buf . push_str (indent) ; } } lit_kind . write_end (& mut buf) . unwrap () ; if matches ! (lit_kind , StrLitKind :: Raw (_)) { buf . push (']') ; } buf }
    };
}

format_patch!();