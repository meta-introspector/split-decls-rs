macro_rules! deps {
    () => {
        SysStack!();
        StackError!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl SysStack { # [doc = " Creates a (non-owning) representation of some stack memory."] # [doc = ""] # [doc = " It is unsafe because it is your responsibility to make sure that `top` and `bottom` are valid"] # [doc = " addresses."] # [inline] pub unsafe fn new (top : * mut c_void , bottom : * mut c_void) -> SysStack { debug_assert ! (top >= bottom) ; SysStack { top , bottom } } # [doc = " Returns the top of the stack from which on it grows downwards towards bottom()."] # [inline] pub fn top (& self) -> * mut c_void { self . top } # [doc = " Returns the bottom of the stack and thus it's end."] # [inline] pub fn bottom (& self) -> * mut c_void { self . bottom } # [doc = " Returns the size of the stack between top() and bottom()."] # [inline] pub fn len (& self) -> usize { self . top as usize - self . bottom as usize } # [doc = " Returns the minimal stack size allowed by the current platform."] # [inline] pub fn min_size () -> usize { sys :: min_stack_size () } # [doc = " Allocates a new stack of `size`."] fn allocate (mut size : usize , protected : bool) -> Result < SysStack , StackError > { let page_size = sys :: page_size () ; let min_stack_size = sys :: min_stack_size () ; let max_stack_size = sys :: max_stack_size () ; let add_shift = i32 :: from (protected) ; let add = page_size << add_shift ; if size < min_stack_size { size = min_stack_size ; } size = (size - 1) & ! (page_size . overflowing_sub (1) . 0) ; if let Some (size) = size . checked_add (add) { if size <= max_stack_size { let mut ret = unsafe { sys :: allocate_stack (size) } ; if protected { if let Ok (stack) = ret { ret = unsafe { sys :: protect_stack (& stack) } ; } } return ret . map_err (StackError :: IoError) ; } } Err (StackError :: ExceedsMaximumSize (max_stack_size - add)) } }
    };
}

impl_65!();