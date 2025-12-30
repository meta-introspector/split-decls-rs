// Generated macro for impl_56 (impl)
macro_rules! Depcrate_linuximpl_56 {
() => {
// Module: crate::linux
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a > SegmentTrait for Segment < 'a > { type SharedLibrary = SharedLibrary < 'a > ; fn name (& self) -> & str { unsafe { match self . phdr . as_ref () . unwrap () . p_type { libc :: PT_NULL => "NULL" , libc :: PT_LOAD => "LOAD" , libc :: PT_DYNAMIC => "DYNAMIC" , libc :: PT_INTERP => "INTERP" , libc :: PT_NOTE => "NOTE" , libc :: PT_SHLIB => "SHLIB" , libc :: PT_PHDR => "PHDR" , libc :: PT_TLS => "TLS" , libc :: PT_GNU_EH_FRAME => "GNU_EH_FRAME" , libc :: PT_GNU_STACK => "GNU_STACK" , libc :: PT_GNU_RELRO => "GNU_RELRO" , _ => "(unknown segment type)" , } } } # [inline] fn is_code (& self) -> bool { let hdr = self . phdr () ; hdr . p_type == libc :: PT_LOAD && (hdr . p_flags & 0x1) != 0 } # [inline] fn is_load (& self) -> bool { self . phdr () . p_type == libc :: PT_LOAD } # [inline] fn stated_virtual_memory_address (& self) -> Svma { Svma (self . phdr () . p_vaddr as _) } # [inline] fn len (& self) -> usize { self . phdr () . p_memsz as _ } }
};
}
