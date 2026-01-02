mkuse!{use std :: cell :: Cell ;}
mkuse!{use std :: ptr ;}
mkitem!{thread_local ! (pub static TLV : Cell <* const () > = const { Cell :: new (ptr :: null ()) }) ;}
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub (crate) struct Tlv (pub (crate) * const ()) ;}}
mkitem!{mkimpl!{impl Tlv { # [inline] pub (crate) fn null () -> Self { Self (ptr :: null ()) } }}}
mkitem!{mkimpl!{unsafe impl Sync for Tlv { }}}
mkitem!{mkimpl!{unsafe impl Send for Tlv { }}}

macro_rules! set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set in module {}", module_path!());
    };
}

mkfn!{
    set_introspect!();
    # [doc = " Sets the current thread-local value"] # [inline] pub (crate) fn set (value : Tlv) { TLV . with (| tlv | tlv . set (value . 0)) ; }
}

macro_rules! get_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get in module {}", module_path!());
    };
}

mkfn!{
    get_introspect!();
    # [doc = " Returns the current thread-local value"] # [inline] pub (crate) fn get () -> Tlv { TLV . with (| tlv | Tlv (tlv . get ())) }
}