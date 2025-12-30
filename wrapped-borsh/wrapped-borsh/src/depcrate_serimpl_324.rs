// Generated macro for impl_324 (impl)
macro_rules! Depcrate_serimpl_324 {
() => {
// Module: crate::ser
// Provides: {"impl_324"}
// Dependencies: {}
impl < T > BorshSerialize for core :: cell :: RefCell < T > where T : BorshSerialize + Sized , { fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { match self . try_borrow () { Ok (ref value) => value . serialize (writer) , Err (_) => Err (Error :: new (ErrorKind :: Other , "already mutably borrowed")) , } } }
};
}
