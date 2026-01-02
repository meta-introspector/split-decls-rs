mkuse!{use super :: mystd :: ffi :: OsString ;}
mkuse!{use super :: mystd :: fs :: File ;}
mkuse!{use super :: mystd :: io :: Read ;}
mkuse!{use alloc :: string :: String ;}
mkuse!{use alloc :: vec :: Vec ;}
mkuse!{use core :: str :: FromStr ;}
mkitem!{mkstruct!{# [derive (PartialEq , Eq , Debug)] pub (super) struct MapsEntry { # [doc = " start (inclusive) and limit (exclusive) of address range."] address : (usize , usize) , # [doc = " The perms field are the permissions for the entry"] # [doc = ""] # [doc = " r = read"] # [doc = " w = write"] # [doc = " x = execute"] # [doc = " s = shared"] # [doc = " p = private (copy on write)"] perms : [char ; 4] , # [doc = " Offset into the file (or \"whatever\")."] offset : u64 , # [doc = " device (major, minor)"] dev : (usize , usize) , # [doc = " inode on the device. 0 indicates that no inode is associated with the memory region (e.g. uninitalized data aka BSS)."] inode : usize , # [doc = " Usually the file backing the mapping."] # [doc = ""] # [doc = " Note: The man page for proc includes a note about \"coordination\" by"] # [doc = " using readelf to see the Offset field in ELF program headers. pnkfelix"] # [doc = " is not yet sure if that is intended to be a comment on pathname, or what"] # [doc = " form/purpose such coordination is meant to have."] # [doc = ""] # [doc = " There are also some pseudo-paths:"] # [doc = " \"[stack]\": The initial process's (aka main thread's) stack."] # [doc = " \"[stack:<tid>]\": a specific thread's stack. (This was only present for a limited range of Linux verisons; it was determined to be too expensive to provide.)"] # [doc = " \"[vdso]\": Virtual dynamically linked shared object"] # [doc = " \"[heap]\": The process's heap"] # [doc = ""] # [doc = " The pathname can be blank, which means it is an anonymous mapping"] # [doc = " obtained via mmap."] # [doc = ""] # [doc = " Newlines in pathname are replaced with an octal escape sequence."] # [doc = ""] # [doc = " The pathname may have \"(deleted)\" appended onto it if the file-backed"] # [doc = " path has been deleted."] # [doc = ""] # [doc = " Note that modifications like the latter two indicated above imply that"] # [doc = " in general the pathname may be ambiguous. (I.e. you cannot tell if the"] # [doc = " denoted filename actually ended with the text \"(deleted)\", or if that"] # [doc = " was added by the maps rendering."] pathname : OsString , }}}

macro_rules! parse_maps_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_maps in module {}", module_path!());
    };
}

mkfn!{
    parse_maps_introspect!();
    pub (super) fn parse_maps () -> Result < Vec < MapsEntry > , & 'static str > { let mut v = Vec :: new () ; let mut proc_self_maps = File :: open ("/proc/self/maps") . map_err (| _ | "Couldn't open /proc/self/maps") ? ; let mut buf = String :: new () ; let _bytes_read = proc_self_maps . read_to_string (& mut buf) . map_err (| _ | "Couldn't read /proc/self/maps") ? ; for line in buf . lines () { v . push (line . parse () ?) ; } Ok (v) }
}
mkitem!{mkimpl!{impl MapsEntry { pub (super) fn pathname (& self) -> & OsString { & self . pathname } pub (super) fn ip_matches (& self , ip : usize) -> bool { self . address . 0 <= ip && ip < self . address . 1 } # [cfg (target_os = "android")] pub (super) fn offset (& self) -> u64 { self . offset } }}}
mkitem!{mkimpl!{impl FromStr for MapsEntry { type Err = & 'static str ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let (range_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if range_str . is_empty () { return Err ("Couldn't find address") ; } let (perms_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if perms_str . is_empty () { return Err ("Couldn't find permissions") ; } let (offset_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if offset_str . is_empty () { return Err ("Couldn't find offset") ; } let (dev_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if dev_str . is_empty () { return Err ("Couldn't find dev") ; } let (inode_str , s) = s . trim_start () . split_once (' ') . unwrap_or ((s , "")) ; if inode_str . is_empty () { return Err ("Couldn't find inode") ; } let pathname_str = s . trim_start () ; let hex = | s | usize :: from_str_radix (s , 16) . map_err (| _ | "Couldn't parse hex number") ; let hex64 = | s | u64 :: from_str_radix (s , 16) . map_err (| _ | "Couldn't parse hex number") ; let address = if let Some ((start , limit)) = range_str . split_once ('-') { (hex (start) ? , hex (limit) ?) } else { return Err ("Couldn't parse address range") ; } ; let perms : [char ; 4] = { let mut chars = perms_str . chars () ; let mut c = | | chars . next () . ok_or ("insufficient perms") ; let perms = [c () ? , c () ? , c () ? , c () ?] ; if chars . next () . is_some () { return Err ("too many perms") ; } perms } ; let offset = hex64 (offset_str) ? ; let dev = if let Some ((major , minor)) = dev_str . split_once (':') { (hex (major) ? , hex (minor) ?) } else { return Err ("Couldn't parse dev") ; } ; let inode = hex (inode_str) ? ; let pathname = pathname_str . into () ; Ok (MapsEntry { address , perms , offset , dev , inode , pathname , }) } }}}

macro_rules! check_maps_entry_parsing_64bit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_maps_entry_parsing_64bit in module {}", module_path!());
    };
}

mkfn!{
    check_maps_entry_parsing_64bit_introspect!();
    # [cfg (target_pointer_width = "64")] # [test] fn check_maps_entry_parsing_64bit () { assert_eq ! ("ffffffffff600000-ffffffffff601000 --xp 00000000 00:00 0                  \
                [vsyscall]" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xffffffffff600000 , 0xffffffffff601000) , perms : ['-' , '-' , 'x' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : "[vsyscall]" . into () , }) ; assert_eq ! ("7f5985f46000-7f5985f48000 rw-p 00039000 103:06 76021795                  \
                /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0x7f5985f46000 , 0x7f5985f48000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00039000 , dev : (0x103 , 0x06) , inode : 0x76021795 , pathname : "/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2" . into () , }) ; assert_eq ! ("35b1a21000-35b1a22000 rw-p 00000000 00:00 0" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0x35b1a21000 , 0x35b1a22000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : Default :: default () , }) ; }
}

macro_rules! check_maps_entry_parsing_32bit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_maps_entry_parsing_32bit in module {}", module_path!());
    };
}

mkfn!{
    check_maps_entry_parsing_32bit_introspect!();
    # [test] fn check_maps_entry_parsing_32bit () { assert_eq ! ("08056000-08077000 rw-p 00000000 00:00 0          \
                [heap]" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0x08056000 , 0x08077000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : "[heap]" . into () , }) ; assert_eq ! ("b7c79000-b7e02000 r--p 00000000 08:01 60662705   \
                /usr/lib/locale/locale-archive" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/usr/lib/locale/locale-archive" . into () , }) ; assert_eq ! ("b7e02000-b7e03000 rw-p 00000000 00:00 0" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7e02000 , 0xb7e03000) , perms : ['r' , 'w' , '-' , 'p'] , offset : 0x00000000 , dev : (0x00 , 0x00) , inode : 0x0 , pathname : Default :: default () , }) ; assert_eq ! ("b7c79000-b7e02000 r--p 00000000 08:01 60662705   \
                /executable/path/with some spaces" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/executable/path/with some spaces" . into () , }) ; assert_eq ! ("b7c79000-b7e02000 r--p 00000000 08:01 60662705   \
                /executable/path/with  multiple-continuous    spaces  " . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/executable/path/with  multiple-continuous    spaces  " . into () , }) ; assert_eq ! ("  b7c79000-b7e02000  r--p  00000000  08:01  60662705   \
                /executable/path/starts-with-spaces" . parse ::< MapsEntry > () . unwrap () , MapsEntry { address : (0xb7c79000 , 0xb7e02000) , perms : ['r' , '-' , '-' , 'p'] , offset : 0x00000000 , dev : (0x08 , 0x01) , inode : 0x60662705 , pathname : "/executable/path/starts-with-spaces" . into () , }) ; }
}