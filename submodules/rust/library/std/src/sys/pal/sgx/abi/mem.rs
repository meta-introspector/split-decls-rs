mkuse!{use core :: arch :: asm ;}

macro_rules! rel_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rel_ptr in module {}", module_path!());
    };
}

mkfn!{
    rel_ptr_introspect!();
    # [inline (always)] pub (crate) unsafe fn rel_ptr < T > (offset : u64) -> * const T { (image_base () + offset) as * const T }
}

macro_rules! rel_ptr_mut_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rel_ptr_mut in module {}", module_path!());
    };
}

mkfn!{
    rel_ptr_mut_introspect!();
    # [inline (always)] pub (crate) unsafe fn rel_ptr_mut < T > (offset : u64) -> * mut T { (image_base () + offset) as * mut T }
}
mkitem!{unsafe extern "C" { static ENCLAVE_SIZE : usize ; static HEAP_BASE : u64 ; static HEAP_SIZE : usize ; }}

macro_rules! heap_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function heap_base in module {}", module_path!());
    };
}

mkfn!{
    heap_base_introspect!();
    # [doc = " Returns the base memory address of the heap"] pub (crate) fn heap_base () -> * const u8 { unsafe { rel_ptr_mut (HEAP_BASE) } }
}

macro_rules! heap_size_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function heap_size in module {}", module_path!());
    };
}

mkfn!{
    heap_size_introspect!();
    # [doc = " Returns the size of the heap"] pub (crate) fn heap_size () -> usize { unsafe { HEAP_SIZE } }
}

macro_rules! image_base_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function image_base in module {}", module_path!());
    };
}

mkfn!{
    image_base_introspect!();
    # [doc = " Returns address at which current enclave is loaded."] # [inline (always)] # [unstable (feature = "sgx_platform" , issue = "56975")] pub fn image_base () -> u64 { let base : u64 ; unsafe { asm ! ("lea IMAGE_BASE(%rip), {}" , lateout (reg) base , options (att_syntax , nostack , preserves_flags , nomem , pure) ,) } ; base }
}

macro_rules! is_enclave_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_enclave_range in module {}", module_path!());
    };
}

mkfn!{
    is_enclave_range_introspect!();
    # [doc = " Returns `true` if the specified memory range is in the enclave."] # [doc = ""] # [doc = " For safety, this function also checks whether the range given overflows,"] # [doc = " returning `false` if so."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub fn is_enclave_range (p : * const u8 , len : usize) -> bool { let start = p as usize ; let end = if len == 0 { start } else if let Some (end) = start . checked_add (len - 1) { end } else { return false ; } ; let base = image_base () as usize ; start >= base && end <= base + (unsafe { ENCLAVE_SIZE } - 1) }
}

macro_rules! is_user_range_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_user_range in module {}", module_path!());
    };
}

mkfn!{
    is_user_range_introspect!();
    # [doc = " Returns `true` if the specified memory range is in userspace."] # [doc = ""] # [doc = " For safety, this function also checks whether the range given overflows,"] # [doc = " returning `false` if so."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub fn is_user_range (p : * const u8 , len : usize) -> bool { let start = p as usize ; let end = if len == 0 { start } else if let Some (end) = start . checked_add (len - 1) { end } else { return false ; } ; let base = image_base () as usize ; end < base || start > base + (unsafe { ENCLAVE_SIZE } - 1) }
}