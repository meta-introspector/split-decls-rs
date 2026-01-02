mkuse!{use crate :: alloc :: { self , Layout } ;}
mkuse!{use crate :: ptr ;}
mkitem!{# [used] static FORCE_CODEGEN_OF_CABI_REALLOC : unsafe extern "C" fn (* mut u8 , usize , usize , usize ,) -> * mut u8 = cabi_realloc ;}

macro_rules! cabi_realloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cabi_realloc in module {}", module_path!());
    };
}

mkfn!{
    cabi_realloc_introspect!();
    # [linkage = "weak"] # [unsafe (no_mangle)] pub unsafe extern "C" fn cabi_realloc (old_ptr : * mut u8 , old_len : usize , align : usize , new_len : usize ,) -> * mut u8 { let layout ; let ptr = if old_len == 0 { if new_len == 0 { return ptr :: without_provenance_mut (align) ; } layout = Layout :: from_size_align_unchecked (new_len , align) ; alloc :: alloc (layout) } else { debug_assert_ne ! (new_len , 0 , "non-zero old_len requires non-zero new_len!") ; layout = Layout :: from_size_align_unchecked (old_len , align) ; alloc :: realloc (old_ptr , layout , new_len) } ; if ptr . is_null () { if cfg ! (debug_assertions) { alloc :: handle_alloc_error (layout) ; } else { super :: abort_internal () ; } } return ptr ; }
}