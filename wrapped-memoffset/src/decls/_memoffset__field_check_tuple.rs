macro_rules! _memoffset__field_check_tuple {
    () => {
        # [cfg (not (allow_clippy))] # [macro_export] # [doc (hidden)] macro_rules ! _memoffset__field_check_tuple { ($ type : ty , $ field : tt) => { let (_ , ..) : $ type ; } ; }
    };
}

_memoffset__field_check_tuple!();