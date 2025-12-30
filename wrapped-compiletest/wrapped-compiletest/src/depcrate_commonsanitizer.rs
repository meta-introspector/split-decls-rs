// Generated macro for Sanitizer (enum)
macro_rules! Depcrate_commonSanitizer {
() => {
// Module: crate::common
// Provides: {"Sanitizer"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , serde :: Deserialize)] # [serde (rename_all = "kebab-case")] pub enum Sanitizer { Address , Cfi , Dataflow , Kcfi , KernelAddress , Leak , Memory , Memtag , Safestack , ShadowCallStack , Thread , Hwaddress , }
};
}
