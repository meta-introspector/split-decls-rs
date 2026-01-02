mkuse!{use super :: { BytesOrWideString , ResolveWhat , SymbolName } ;}
mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: marker ;}

macro_rules! resolve_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve in module {}", module_path!());
    };
}

mkfn!{
    resolve_introspect!();
    pub unsafe fn resolve (_addr : ResolveWhat < '_ > , _cb : & mut dyn FnMut (& super :: Symbol)) { }
}
mkitem!{mkstruct!{pub struct Symbol < 'a > { _marker : marker :: PhantomData < & 'a i32 > , }}}
mkitem!{mkimpl!{impl Symbol < '_ > { pub fn name (& self) -> Option < SymbolName < '_ > > { None } pub fn addr (& self) -> Option < * mut c_void > { None } pub fn filename_raw (& self) -> Option < BytesOrWideString < '_ > > { None } # [cfg (feature = "std")] pub fn filename (& self) -> Option < & :: std :: path :: Path > { None } pub fn lineno (& self) -> Option < u32 > { None } pub fn colno (& self) -> Option < u32 > { None } }}}

macro_rules! clear_symbol_cache_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clear_symbol_cache in module {}", module_path!());
    };
}

mkfn!{
    clear_symbol_cache_introspect!();
    pub unsafe fn clear_symbol_cache () { }
}