macro_rules! deps {
    () => {
        StatusAnd!();
        Float!();
        Round!();
        ParseError!();
    };
}

macro_rules! float_common_impls {
    () => {
        deps!();
        macro_rules ! float_common_impls { ($ ty : ident <$ t : tt >) => { impl <$ t > Default for $ ty <$ t > where Self : Float , { # [inline] fn default () -> Self { Self :: ZERO } } impl <$ t > :: core :: str :: FromStr for $ ty <$ t > where Self : Float , { type Err = ParseError ; # [inline] fn from_str (s : & str) -> Result < Self , ParseError > { Self :: from_str_r (s , Round :: NearestTiesToEven) . map (| x | x . value) } } impl <$ t > :: core :: ops :: Add for $ ty <$ t > where Self : Float , { type Output = StatusAnd < Self >; # [inline] fn add (self , rhs : Self) -> StatusAnd < Self > { self . add_r (rhs , Round :: NearestTiesToEven) } } impl <$ t > :: core :: ops :: Sub for $ ty <$ t > where Self : Float , { type Output = StatusAnd < Self >; # [inline] fn sub (self , rhs : Self) -> StatusAnd < Self > { self . sub_r (rhs , Round :: NearestTiesToEven) } } impl <$ t > :: core :: ops :: Mul for $ ty <$ t > where Self : Float , { type Output = StatusAnd < Self >; # [inline] fn mul (self , rhs : Self) -> StatusAnd < Self > { self . mul_r (rhs , Round :: NearestTiesToEven) } } impl <$ t > :: core :: ops :: Div for $ ty <$ t > where Self : Float , { type Output = StatusAnd < Self >; # [inline] fn div (self , rhs : Self) -> StatusAnd < Self > { self . div_r (rhs , Round :: NearestTiesToEven) } } impl <$ t > :: core :: ops :: Rem for $ ty <$ t > where Self : Float , { type Output = StatusAnd < Self >; # [inline] fn rem (self , rhs : Self) -> StatusAnd < Self > { self . c_fmod (rhs) } } impl <$ t > :: core :: ops :: AddAssign for $ ty <$ t > where Self : Float , { # [inline] fn add_assign (& mut self , rhs : Self) { * self = (* self + rhs) . value ; } } impl <$ t > :: core :: ops :: SubAssign for $ ty <$ t > where Self : Float , { # [inline] fn sub_assign (& mut self , rhs : Self) { * self = (* self - rhs) . value ; } } impl <$ t > :: core :: ops :: MulAssign for $ ty <$ t > where Self : Float , { # [inline] fn mul_assign (& mut self , rhs : Self) { * self = (* self * rhs) . value ; } } impl <$ t > :: core :: ops :: DivAssign for $ ty <$ t > where Self : Float , { # [inline] fn div_assign (& mut self , rhs : Self) { * self = (* self / rhs) . value ; } } impl <$ t > :: core :: ops :: RemAssign for $ ty <$ t > where Self : Float , { # [inline] fn rem_assign (& mut self , rhs : Self) { * self = (* self % rhs) . value ; } } } ; }
    };
}

float_common_impls!();