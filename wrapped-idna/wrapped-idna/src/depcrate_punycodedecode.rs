// Generated macro for Decode (struct)
macro_rules! Depcrate_punycodeDecode {
() => {
// Module: crate::punycode
// Provides: {"Decode"}
// Dependencies: {}
pub (crate) struct Decode < 'a , T , C > where T : PunycodeCodeUnit + Copy , C : PunycodeCaller , { base : core :: slice :: Iter < 'a , T > , pub (crate) insertions : & 'a [(usize , char)] , inserted : usize , position : usize , len : usize , phantom : PhantomData < C > , }
};
}
