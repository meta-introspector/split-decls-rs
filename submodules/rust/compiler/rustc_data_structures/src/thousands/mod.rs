mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! format_with_underscores_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_with_underscores in module {}", module_path!());
    };
}

mkfn!{
    format_with_underscores_introspect!();
    fn format_with_underscores (mut s : String) -> String { let start = if s . starts_with ('-') { 1 } else { 0 } ; let non_digit = s [start ..] . find (| c : char | ! c . is_digit (10)) ; let end = if let Some (non_digit) = non_digit { start + non_digit } else { s . len () } ; let mut i = end ; while i > start + 3 { i -= 3 ; s . insert (i , '_') ; } s }
}

macro_rules! usize_with_underscores_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usize_with_underscores in module {}", module_path!());
    };
}

mkfn!{
    usize_with_underscores_introspect!();
    # [doc = " Print a `usize` with underscore separators."] pub fn usize_with_underscores (n : usize) -> String { format_with_underscores (format ! ("{n}")) }
}

macro_rules! isize_with_underscores_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function isize_with_underscores in module {}", module_path!());
    };
}

mkfn!{
    isize_with_underscores_introspect!();
    # [doc = " Print an `isize` with underscore separators."] pub fn isize_with_underscores (n : isize) -> String { format_with_underscores (format ! ("{n}")) }
}

macro_rules! f64p1_with_underscores_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function f64p1_with_underscores in module {}", module_path!());
    };
}

mkfn!{
    f64p1_with_underscores_introspect!();
    # [doc = " Print an `f64` with precision 1 (one decimal place) and underscore separators."] pub fn f64p1_with_underscores (n : f64) -> String { format_with_underscores (format ! ("{n:.1}")) }
}