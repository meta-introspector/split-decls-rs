mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use super :: super :: backtrace :: miri :: { Frame , resolve_addr } ;}
mkuse!{use super :: BytesOrWideString ;}
mkuse!{use super :: { ResolveWhat , SymbolName } ;}

macro_rules! resolve_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function resolve in module {}", module_path!());
    };
}

mkfn!{
    resolve_introspect!();
    pub unsafe fn resolve (what : ResolveWhat < '_ > , cb : & mut dyn FnMut (& super :: Symbol)) { let sym = match what { ResolveWhat :: Address (addr) => Symbol { inner : resolve_addr (addr) , _unused : PhantomData , } , ResolveWhat :: Frame (frame) => Symbol { inner : frame . inner . clone () , _unused : PhantomData , } , } ; cb (& super :: Symbol { inner : sym }) }
}
mkitem!{mkstruct!{pub struct Symbol < 'a > { inner : Frame , _unused : PhantomData < & 'a () > , }}}
mkitem!{mkimpl!{impl < 'a > Symbol < 'a > { pub fn name (& self) -> Option < SymbolName < '_ > > { Some (SymbolName :: new (& self . inner . inner . name)) } pub fn addr (& self) -> Option < * mut c_void > { Some (self . inner . addr) } pub fn filename_raw (& self) -> Option < BytesOrWideString < '_ > > { Some (BytesOrWideString :: Bytes (& self . inner . inner . filename)) } pub fn lineno (& self) -> Option < u32 > { Some (self . inner . inner . lineno) } pub fn colno (& self) -> Option < u32 > { Some (self . inner . inner . colno) } # [cfg (feature = "std")] pub fn filename (& self) -> Option < & std :: path :: Path > { Some (std :: path :: Path :: new (core :: str :: from_utf8 (& self . inner . inner . filename) . unwrap () ,)) } }}}

macro_rules! clear_symbol_cache_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clear_symbol_cache in module {}", module_path!());
    };
}

mkfn!{
    clear_symbol_cache_introspect!();
    pub unsafe fn clear_symbol_cache () { }
}