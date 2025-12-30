// Generated macro for impl_1626 (impl)
macro_rules! Depcrate_build_elfimpl_1626 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1626"}
// Dependencies: {}
impl < 'data > Dynamic < 'data > { # [doc = " The `d_tag` field in the dynamic entry."] # [doc = ""] # [doc = " One of the `DT_*` values."] pub fn tag (& self) -> u32 { match self { Dynamic :: Auto { tag } => * tag , Dynamic :: Integer { tag , .. } => * tag , Dynamic :: String { tag , .. } => * tag , } } }
};
}
