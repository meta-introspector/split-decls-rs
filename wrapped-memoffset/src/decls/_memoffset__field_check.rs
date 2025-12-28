macro_rules! _memoffset__field_check {
    () => {
        # [cfg (not (allow_clippy))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset__field_check { ($ type : path , $ field : tt) => { let $ type { $ field : _ , .. } ; } ; }
    };
}

_memoffset__field_check!();