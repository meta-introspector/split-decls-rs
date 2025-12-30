// Generated macro for AndroidSystemProperties (struct)
macro_rules! DepcrateAndroidSystemProperties {
() => {
// Module: crate
// Provides: {"AndroidSystemProperties"}
// Dependencies: {}
# [derive (Debug)] # [doc = " An object that can retrieve android system properties."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use android_system_properties::AndroidSystemProperties;"] # [doc = ""] # [doc = " let properties = AndroidSystemProperties::new();"] # [doc = ""] # [doc = " if let Some(value) = properties.get(\"persist.sys.timezone\") {"] # [doc = "    println!(\"{}\", value);"] # [doc = " }"] # [doc = " ```"] pub struct AndroidSystemProperties { libc_so : * mut c_void , get_fn : Option < SystemPropertyGetFn > , find_fn : Option < SystemPropertyFindFn > , read_callback_fn : Option < SystemPropertyReadCallbackFn > , }
};
}
