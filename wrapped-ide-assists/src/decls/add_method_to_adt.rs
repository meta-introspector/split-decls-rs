macro_rules! add_method_to_adt {
    () => {
        pub (crate) fn add_method_to_adt (builder : & mut SourceChangeBuilder , adt : & ast :: Adt , impl_def : Option < ast :: Impl > , method : & str ,) { let mut buf = String :: with_capacity (method . len () + 2) ; if impl_def . is_some () { buf . push ('\n') ; } buf . push_str (method) ; let start_offset = impl_def . and_then (| impl_def | find_impl_block_end (impl_def , & mut buf)) . unwrap_or_else (| | { buf = generate_impl_text (adt , & buf) ; adt . syntax () . text_range () . end () }) ; builder . insert (start_offset , buf) ; }
    };
}

add_method_to_adt!()