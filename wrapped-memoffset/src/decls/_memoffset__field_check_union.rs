macro_rules! _memoffset__field_check_union {
    () => {
        # [cfg (not (allow_clippy))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset__field_check_union { ($ type : path , $ field : tt) => { # [allow (unused_unsafe)] unsafe { let $ type { $ field : _ } ; } } ; }
    };
}

_memoffset__field_check_union!()