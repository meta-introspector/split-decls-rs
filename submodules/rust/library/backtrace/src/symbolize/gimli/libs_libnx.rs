mkuse!{use super :: { Library , LibrarySegment } ;}
mkuse!{use alloc :: vec :: Vec ;}

macro_rules! native_libraries_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function native_libraries in module {}", module_path!());
    };
}

mkfn!{
    native_libraries_introspect!();
    pub (super) fn native_libraries () -> Vec < Library > { unsafe extern "C" { static __start__ : u8 ; } let bias = core :: ptr :: addr_of ! (__start__) as usize ; let mut ret = Vec :: new () ; let mut segments = Vec :: new () ; segments . push (LibrarySegment { stated_virtual_memory_address : 0 , len : usize :: max_value () - bias , }) ; let path = "romfs:/debug_info.elf" ; ret . push (Library { name : path . into () , segments , bias , }) ; ret }
}