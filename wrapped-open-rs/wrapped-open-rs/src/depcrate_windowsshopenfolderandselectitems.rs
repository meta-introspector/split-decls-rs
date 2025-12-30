// Generated macro for SHOpenFolderAndSelectItems (function)
macro_rules! Depcrate_windowsSHOpenFolderAndSelectItems {
() => {
// Module: crate::windows
// Provides: {"SHOpenFolderAndSelectItems"}
// Dependencies: {}
# [doc = " Opens a Windows Explorer window with specified items in a particular folder selected."] # [doc = ""] # [doc = " <https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shopenfolderandselectitems>"] # [allow (non_snake_case)] # [cfg (feature = "shellexecute-on-windows")] pub unsafe fn SHOpenFolderAndSelectItems (pidlfolder : * const ffi :: ITEMIDLIST , apidl : Option < & [* const ffi :: ITEMIDLIST] > , dwflags : u32 ,) -> std :: io :: Result < () > { use std :: convert :: TryInto ; match ffi :: SHOpenFolderAndSelectItems (pidlfolder , apidl . as_deref () . map_or (0 , | slice | slice . len () . try_into () . unwrap ()) , core :: mem :: transmute (apidl . as_deref () . map_or (core :: ptr :: null () , | slice | slice . as_ptr ()) ,) , dwflags ,) { 0 => Ok (()) , error_code => Err (std :: io :: Error :: from_raw_os_error (error_code)) , } }
};
}
