macro_rules! set_link_section {
    () => {
        pub (crate) fn set_link_section (llval : & Value , attrs : & CodegenFnAttrs) { let Some (sect) = attrs . link_section else { return } ; let buf = SmallCStr :: new (sect . as_str ()) ; llvm :: set_section (llval , & buf) ; }
    };
}

set_link_section!()