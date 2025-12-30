// Generated macro for impl_23 (impl)
macro_rules! Depcrate_block_apiimpl_23 {
() => {
// Module: crate::block_api
// Provides: {"impl_23"}
// Dependencies: {}
impl < P : Gost94Params > UpdateCore for Gost94Core < P > { # [inline] fn update_blocks (& mut self , blocks : & [TBlock < Self >]) { let len = Self :: BlockSize :: USIZE * blocks . len () ; self . update_n (len) ; blocks . iter () . for_each (| b | self . compress (b . as_ref ())) ; } }
};
}
