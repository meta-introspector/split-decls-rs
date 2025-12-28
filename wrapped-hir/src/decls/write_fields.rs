macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! write_fields {
    () => {
        deps!();
        fn write_fields < 'db > (fields : & [Field] , has_where_clause : bool , limit : usize , in_line : bool , f : & mut HirFormatter < '_ , 'db > ,) -> Result < () , HirDisplayError > { let count = fields . len () . min (limit) ; let (indent , separator) = if in_line { ("" , ' ') } else { ("    " , '\n') } ; f . write_char (if ! has_where_clause { ' ' } else { separator }) ? ; if count == 0 { f . write_str (if fields . is_empty () { "{}" } else { "{ /* … */ }" }) ? ; } else { f . write_char ('{') ? ; if ! fields . is_empty () { f . write_char (separator) ? ; for field in & fields [.. count] { f . write_str (indent) ? ; field . hir_fmt (f) ? ; write ! (f , ",{separator}") ? ; } if fields . len () > count { write ! (f , "{indent}/* … */{separator}") ? ; } } f . write_str ("}") ? ; } Ok (()) }
    };
}

write_fields!();