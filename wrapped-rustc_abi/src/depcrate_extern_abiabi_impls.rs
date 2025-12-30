// Generated macro for abi_impls (macro)
macro_rules! Depcrate_extern_abiabi_impls {
() => {
// Module: crate::extern_abi
// Provides: {"abi_impls"}
// Dependencies: {}
macro_rules ! abi_impls { ($ e_name : ident = { $ ($ variant : ident $ ({ unwind : $ uw : literal }) ? =><= $ tok : literal ,) * }) => { impl $ e_name { pub const ALL_VARIANTS : & [Self] = & [$ ($ e_name ::$ variant $ ({ unwind : $ uw }) *,) *] ; pub const fn as_str (& self) -> &'static str { match self { $ ($ e_name ::$ variant $ ({ unwind : $ uw }) * => $ tok ,) * } } } impl :: core :: str :: FromStr for $ e_name { type Err = AbiFromStrErr ; fn from_str (s : & str) -> Result <$ e_name , Self :: Err > { match s { $ ($ tok => Ok ($ e_name ::$ variant $ ({ unwind : $ uw }) *) ,) * _ => Err (AbiFromStrErr :: Unknown) , } } } } }
};
}
