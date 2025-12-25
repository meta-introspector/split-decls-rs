use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns true if there is an MSYS tty on the given handle.
#[cfg(windows)]
unsafe fn msys_tty_on(handle: HANDLE) -> bool {
    use std::ffi::c_void;
    use windows_sys::Win32::{
        Foundation::MAX_PATH,
        Storage::FileSystem::{
            FileNameInfo, GetFileInformationByHandleEx, GetFileType, FILE_TYPE_PIPE,
        },
    };
    if GetFileType(handle) != FILE_TYPE_PIPE {
        return false;
    }
    /// Mirrors windows_sys::Win32::Storage::FileSystem::FILE_NAME_INFO, giving
    /// it a fixed length that we can stack allocate
    #[repr(C)]
    #[allow(non_snake_case)]
    struct FILE_NAME_INFO {
        FileNameLength: u32,
        FileName: [u16; MAX_PATH as usize],
    }
    let mut name_info = FILE_NAME_INFO {
        FileNameLength: 0,
        FileName: [0; MAX_PATH as usize],
    };
    let res = GetFileInformationByHandleEx(
        handle,
        FileNameInfo,
        &mut name_info as *mut _ as *mut c_void,
        std::mem::size_of::<FILE_NAME_INFO>() as u32,
    );
    if res == 0 {
        return false;
    }
    let s = match name_info.FileName.get(..name_info.FileNameLength as usize / 2) {
        None => return false,
        Some(s) => s,
    };
    let name = String::from_utf16_lossy(s);
    let name = name.rsplit('\\').next().unwrap_or(&name);
    let is_msys = name.starts_with("msys-") || name.starts_with("cygwin-");
    let is_pty = name.contains("-pty");
    is_msys && is_pty
}
