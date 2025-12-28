macro_rules! demangle_line {
    () => {
        # [cfg (feature = "std")] fn demangle_line (line : & str , output : & mut impl std :: io :: Write , include_hash : bool ,) -> std :: io :: Result < () > { let mut head = 0 ; while head < line . len () { let next_head = match (line [head ..] . find ("_ZN") , line [head ..] . find ("_R")) { (Some (idx) , None) | (None , Some (idx)) => head + idx , (Some (idx1) , Some (idx2)) => head + idx1 . min (idx2) , (None , None) => { line . len () } } ; output . write_all (line [head .. next_head] . as_bytes ()) ? ; head = next_head ; let match_end = line [head ..] . find (| ch : char | ! (ch == '$' || ch == '.' || ch == '_' || ch . is_ascii_alphanumeric ())) . map (| idx | head + idx) . unwrap_or (line . len ()) ; let mangled = & line [head .. match_end] ; head = head + mangled . len () ; if let Ok (demangled) = try_demangle (mangled) { if include_hash { write ! (output , "{}" , demangled) ? ; } else { write ! (output , "{:#}" , demangled) ? ; } } else { output . write_all (mangled . as_bytes ()) ? ; } } Ok (()) }
    };
}

demangle_line!();