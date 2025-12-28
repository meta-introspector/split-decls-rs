macro_rules! deps {
    () => {
        IsExecutable!();
    };
}

macro_rules! windows {
    () => {
        deps!();
        # [cfg (target_os = "windows")] mod windows { use std :: os :: windows :: ffi :: OsStrExt ; use std :: path :: Path ; use windows_sys :: Win32 :: Storage :: FileSystem :: GetBinaryTypeW ; use super :: IsExecutable ; impl IsExecutable for Path { fn is_executable (& self) -> bool { if ! self . exists () { return false ; } if let Some (pathext) = std :: env :: var_os ("PATHEXT") { if let Some (extension) = self . extension () { let extension = extension . to_string_lossy () ; return pathext . to_string_lossy () . split (';') . filter (| f | f . len () > 1) . any (| ext | { let ext = & ext [1 ..] ; extension . eq_ignore_ascii_case (ext) }) ; } } let windows_string = self . as_os_str () . encode_wide () . chain (Some (0)) . collect :: < Vec < _ > > () ; let windows_string_ptr = windows_string . as_ptr () ; let mut binary_type : u32 = 42 ; let binary_type_ptr = & mut binary_type as * mut u32 ; let ret = unsafe { GetBinaryTypeW (windows_string_ptr , binary_type_ptr) } ; if binary_type_ptr . is_null () { return false ; } if ret != 0 { let binary_type = unsafe { * binary_type_ptr } ; match binary_type { 0 | 1 | 2 | 3 | 4 | 5 | 6 => return true , _ => () , } } false } } }
    };
}

windows!();