// Generated macro for impl_4 (impl)
macro_rules! Depcrate_versionimpl_4 {
() => {
// Module: crate::version
// Provides: {"impl_4"}
// Dependencies: {}
impl JNIVersion { # [doc = " JNI Version 1.1"] pub const V1_1 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_1_1 as u32 , } ; # [doc = " JNI Version 1.2"] pub const V1_2 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_1_2 as u32 , } ; # [doc = " JNI Version 1.4"] pub const V1_4 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_1_4 as u32 , } ; # [doc = " JNI Version 1.6"] pub const V1_6 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_1_6 as u32 , } ; # [doc = " JNI Version 1.8"] pub const V1_8 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_1_8 as u32 , } ; # [doc = " JNI Version 9.0"] pub const V9 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_9 as u32 , } ; # [doc = " JNI Version 10.0"] pub const V10 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_10 as u32 , } ; # [doc = " JNI Version 19.0"] pub const V19 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_19 as u32 , } ; # [doc = " JNI Version 20.0"] pub const V20 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_20 as u32 , } ; # [doc = " JNI Version 21.0"] pub const V21 : Self = JNIVersion { ver : jni_sys :: JNI_VERSION_21 as u32 , } ; # [doc = " Return a version from a raw version constant like [`jni_sys::JNI_VERSION_1_2`]"] pub fn new (ver : jni_sys :: jint) -> Self { Self :: from (ver) } # [doc = " Get the major component of the version number"] pub fn major (& self) -> u16 { ((self . ver & 0x00ff0000) >> 16) as u16 } # [doc = " Get the minor component of the version number"] pub fn minor (& self) -> u16 { (self . ver & 0xff) as u16 } }
};
}
