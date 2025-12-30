// Generated macro for impl_1003 (impl)
macro_rules! Depcrate_themeimpl_1003 {
() => {
// Module: crate::theme
// Provides: {"impl_1003"}
// Dependencies: {}
impl FileStyle for FileTypes { fn get_style (& self , file : & File < '_ > , theme : & Theme) -> Option < Style > { # [rustfmt :: skip] return match FileType :: get_file_type (file) { Some (FileType :: Image) => theme . ui . file_type . unwrap_or_default () . image , Some (FileType :: Video) => theme . ui . file_type . unwrap_or_default () . video , Some (FileType :: Music) => theme . ui . file_type . unwrap_or_default () . music , Some (FileType :: Lossless) => theme . ui . file_type . unwrap_or_default () . lossless , Some (FileType :: Crypto) => theme . ui . file_type . unwrap_or_default () . crypto , Some (FileType :: Document) => theme . ui . file_type . unwrap_or_default () . document , Some (FileType :: Compressed) => theme . ui . file_type . unwrap_or_default () . compressed , Some (FileType :: Temp) => theme . ui . file_type . unwrap_or_default () . temp , Some (FileType :: Compiled) => theme . ui . file_type . unwrap_or_default () . compiled , Some (FileType :: Build) => theme . ui . file_type . unwrap_or_default () . build , Some (FileType :: Source) => theme . ui . file_type . unwrap_or_default () . source , None => None } ; } }
};
}
