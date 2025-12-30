// Generated macro for impl_384 (impl)
macro_rules! Depcrateimpl_384 {
() => {
// Module: crate
// Provides: {"impl_384"}
// Dependencies: {}
impl EvaluatedConst { pub fn render (& self , db : & dyn HirDatabase , display_target : DisplayTarget) -> String { format ! ("{}" , self . const_ . display (db , display_target)) } pub fn render_debug (& self , db : & dyn HirDatabase) -> Result < String , MirEvalError > { let data = self . const_ . data (Interner) ; if let TyKind :: Scalar (s) = data . ty . kind (Interner) && matches ! (s , Scalar :: Int (_) | Scalar :: Uint (_)) && let hir_ty :: ConstValue :: Concrete (c) = & data . value && let hir_ty :: ConstScalar :: Bytes (b , _) = & c . interned { let value = u128 :: from_le_bytes (mir :: pad16 (b , false)) ; let value_signed = i128 :: from_le_bytes (mir :: pad16 (b , matches ! (s , Scalar :: Int (_)))) ; let mut result = if let Scalar :: Int (_) = s { value_signed . to_string () } else { value . to_string () } ; if value >= 10 { format_to ! (result , " ({value:#X})") ; return Ok (result) ; } else { return Ok (result) ; } } mir :: render_const_using_debug_impl (db , self . def , & self . const_) } }
};
}
