// Generated macro for Gost94Core (struct)
macro_rules! Depcrate_block_apiGost94Core {
() => {
// Module: crate::block_api
// Provides: {"Gost94Core"}
// Dependencies: {}
# [doc = " Core GOST94 algorithm generic over parameters."] pub struct Gost94Core < P : Gost94Params > { h : Block , n : [u64 ; 4] , sigma : [u64 ; 4] , _m : core :: marker :: PhantomData < P > , }
};
}
