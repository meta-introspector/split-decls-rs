// Generated macro for impl_17 (impl)
macro_rules! Depcrate_levelimpl_17 {
() => {
// Module: crate::level
// Provides: {"impl_17"}
// Dependencies: {}
impl LevelInner { pub (crate) fn style (self , stylesheet : & Stylesheet) -> Style { match self { LevelInner :: Error => stylesheet . error , LevelInner :: Warning => stylesheet . warning , LevelInner :: Info => stylesheet . info , LevelInner :: Note => stylesheet . note , LevelInner :: Help => stylesheet . help , } } }
};
}
