mkuse!{use crate :: ffi :: c_void ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicBool , AtomicPtr , Ordering } ;}
mkitem!{static SYSTEM_TABLE : Atomic < * mut c_void > = AtomicPtr :: new (crate :: ptr :: null_mut ()) ;}
mkitem!{static IMAGE_HANDLE : Atomic < * mut c_void > = AtomicPtr :: new (crate :: ptr :: null_mut ()) ;}
mkitem!{static BOOT_SERVICES_FLAG : Atomic < bool > = AtomicBool :: new (false) ;}

macro_rules! init_globals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_globals in module {}", module_path!());
    };
}

mkfn!{
    init_globals_introspect!();
    # [doc = " Initializes the global System Table and Image Handle pointers."] # [doc = ""] # [doc = " The standard library requires access to the UEFI System Table and the Application Image Handle"] # [doc = " to operate. Those are provided to UEFI Applications via their application entry point. By"] # [doc = " calling `init_globals()`, those pointers are retained by the standard library for future use."] # [doc = " Thus this function must be called before any of the standard library services are used."] # [doc = ""] # [doc = " The pointers are never exposed to any entity outside of this application and it is guaranteed"] # [doc = " that, once the application exited, these pointers are never dereferenced again."] # [doc = ""] # [doc = " Callers are required to ensure the pointers are valid for the entire lifetime of this"] # [doc = " application. In particular, UEFI Boot Services must not be exited while an application with the"] # [doc = " standard library is loaded."] # [doc = ""] # [doc = " # SAFETY"] # [doc = " Calling this function more than once will panic."] pub (crate) unsafe fn init_globals (handle : NonNull < c_void > , system_table : NonNull < c_void >) { IMAGE_HANDLE . compare_exchange (crate :: ptr :: null_mut () , handle . as_ptr () , Ordering :: Release , Ordering :: Acquire ,) . unwrap () ; SYSTEM_TABLE . compare_exchange (crate :: ptr :: null_mut () , system_table . as_ptr () , Ordering :: Release , Ordering :: Acquire ,) . unwrap () ; BOOT_SERVICES_FLAG . store (true , Ordering :: Release) }
}

macro_rules! system_table_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function system_table in module {}", module_path!());
    };
}

mkfn!{
    system_table_introspect!();
    # [doc = " Gets the SystemTable Pointer."] # [doc = ""] # [doc = " If you want to use `BootServices` then please use [`boot_services`] as it performs some"] # [doc = " additional checks."] # [doc = ""] # [doc = " Note: This function panics if the System Table or Image Handle is not initialized."] pub fn system_table () -> NonNull < c_void > { try_system_table () . unwrap () }
}

macro_rules! image_handle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function image_handle in module {}", module_path!());
    };
}

mkfn!{
    image_handle_introspect!();
    # [doc = " Gets the ImageHandle Pointer."] # [doc = ""] # [doc = " Note: This function panics if the System Table or Image Handle is not initialized."] pub fn image_handle () -> NonNull < c_void > { try_image_handle () . unwrap () }
}

macro_rules! boot_services_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function boot_services in module {}", module_path!());
    };
}

mkfn!{
    boot_services_introspect!();
    # [doc = " Gets the BootServices Pointer."] # [doc = ""] # [doc = " This function also checks if `ExitBootServices` has already been called."] pub fn boot_services () -> Option < NonNull < c_void > > { if BOOT_SERVICES_FLAG . load (Ordering :: Acquire) { let system_table : NonNull < r_efi :: efi :: SystemTable > = try_system_table () ? . cast () ; let boot_services = unsafe { (* system_table . as_ptr ()) . boot_services } ; NonNull :: new (boot_services) . map (| x | x . cast ()) } else { None } }
}

macro_rules! try_system_table_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_system_table in module {}", module_path!());
    };
}

mkfn!{
    try_system_table_introspect!();
    # [doc = " Gets the SystemTable Pointer."] # [doc = ""] # [doc = " This function is mostly intended for places where panic is not an option."] pub (crate) fn try_system_table () -> Option < NonNull < c_void > > { NonNull :: new (SYSTEM_TABLE . load (Ordering :: Acquire)) }
}

macro_rules! try_image_handle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_image_handle in module {}", module_path!());
    };
}

mkfn!{
    try_image_handle_introspect!();
    # [doc = " Gets the SystemHandle Pointer."] # [doc = ""] # [doc = " This function is mostly intended for places where panicking is not an option."] pub (crate) fn try_image_handle () -> Option < NonNull < c_void > > { NonNull :: new (IMAGE_HANDLE . load (Ordering :: Acquire)) }
}

macro_rules! disable_boot_services_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function disable_boot_services in module {}", module_path!());
    };
}

mkfn!{
    disable_boot_services_introspect!();
    pub (crate) fn disable_boot_services () { BOOT_SERVICES_FLAG . store (false , Ordering :: Release) }
}