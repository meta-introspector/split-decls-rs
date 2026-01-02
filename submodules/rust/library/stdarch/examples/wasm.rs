mkuse!{use std :: ptr ;}
mkuse!{use core_arch :: arch :: wasm32 :: * ;}
mkitem!{static mut HEAD : * mut * mut u8 = 0 as _ ;}

macro_rules! page_alloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function page_alloc in module {}", module_path!());
    };
}

mkfn!{
    page_alloc_introspect!();
    # [unsafe (no_mangle)] pub unsafe extern "C" fn page_alloc () -> * mut u8 { unsafe { if ! HEAD . is_null () { let next = * HEAD ; let ret = HEAD ; HEAD = next as * mut _ ; return ret as * mut u8 ; } } let ret = memory_grow (0 , 1) ; if ret == usize :: MAX { return ptr :: null_mut () ; } ((ret as u32) * page_size ()) as * mut u8 }
}

macro_rules! page_free_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function page_free in module {}", module_path!());
    };
}

mkfn!{
    page_free_introspect!();
    # [unsafe (no_mangle)] pub unsafe extern "C" fn page_free (page : * mut u8) { let page = page as * mut * mut u8 ; unsafe { * page = HEAD as * mut u8 ; HEAD = page ; } }
}

macro_rules! memory_used_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function memory_used in module {}", module_path!());
    };
}

mkfn!{
    memory_used_introspect!();
    # [unsafe (no_mangle)] pub unsafe extern "C" fn memory_used () -> usize { (page_size () * (memory_size (0) as u32)) as usize }
}

macro_rules! page_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function page_size in module {}", module_path!());
    };
}

mkfn!{
    page_size_introspect!();
    fn page_size () -> u32 { 64 * 1024 }
}