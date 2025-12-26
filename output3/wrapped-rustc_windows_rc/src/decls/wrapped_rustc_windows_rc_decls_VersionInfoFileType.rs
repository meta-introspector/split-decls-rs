use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A subset of the possible values for the `FILETYPE` field in a Windows resource file
///
/// See the `dwFileType` member of [VS_FIXEDFILEINFO](https://learn.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo#members)
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum VersionInfoFileType {
    /// `VFT_APP` - The file is an application.
    App = 0x00000001,
    /// `VFT_DLL` - The file is a dynamic link library.
    Dll = 0x00000002,
}
