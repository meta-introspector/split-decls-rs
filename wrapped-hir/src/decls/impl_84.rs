macro_rules! deps {
    () => {
        EvaluatedConst!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < 'db > EvaluatedConst < 'db > { pub fn render (& self , db : & dyn HirDatabase , display_target : DisplayTarget) -> String { format ! ("{}" , self . const_ . display (db , display_target)) } pub fn render_debug (& self , db : & 'db dyn HirDatabase) -> Result < String , MirEvalError < 'db > > { let kind = self . const_ . kind () ; if let ConstKind :: Value (c) = kind && let ty = c . ty . kind () && let TyKind :: Int (_) | TyKind :: Uint (_) = ty { let b = & c . value . inner () . memory ; let value = u128 :: from_le_bytes (mir :: pad16 (b , false)) ; let value_signed = i128 :: from_le_bytes (mir :: pad16 (b , matches ! (ty , TyKind :: Int (_)))) ; let mut result = if let TyKind :: Int (_) = ty { value_signed . to_string () } else { value . to_string () } ; if value >= 10 { format_to ! (result , " ({value:#X})") ; return Ok (result) ; } else { return Ok (result) ; } } mir :: render_const_using_debug_impl (db , self . def , self . const_ , self . ty) } }
    };
}

impl_84!()