mkuse!{use core :: num :: { NonZero , Saturating , Wrapping } ;}
mkuse!{use crate :: boxed :: Box ;}
mkitem!{mktrait!{# [rustc_specialization_trait] pub (super) unsafe trait IsZero { # [doc = " Whether this value's representation is all zeros,"] # [doc = " or can be represented with all zeroes."] fn is_zero (& self) -> bool ; }}}
mkitem!{macro_rules ! impl_is_zero { ($ t : ty , $ is_zero : expr) => { unsafe impl IsZero for $ t { # [inline] fn is_zero (& self) -> bool { $ is_zero (* self) } } } ; }}
mkitem!{impl_is_zero ! (i8 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (i16 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (i32 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (i64 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (i128 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (isize , | x | x == 0) ;}
mkitem!{impl_is_zero ! (u8 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (u16 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (u32 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (u64 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (u128 , | x | x == 0) ;}
mkitem!{impl_is_zero ! (usize , | x | x == 0) ;}
mkitem!{impl_is_zero ! (bool , | x | x == false) ;}
mkitem!{impl_is_zero ! (char , | x | x == '\0') ;}
mkitem!{impl_is_zero ! (f32 , | x : f32 | x . to_bits () == 0) ;}
mkitem!{impl_is_zero ! (f64 , | x : f64 | x . to_bits () == 0) ;}
mkitem!{mkimpl!{unsafe impl < T : IsZero , const N : usize > IsZero for [T ; N] { # [inline] fn is_zero (& self) -> bool { N <= 16 && self . iter () . all (IsZero :: is_zero) } }}}
mkitem!{macro_rules ! impl_is_zero_tuples { () => { } ; ($ first_arg : ident $ (,$ rest : ident) *) => { unsafe impl <$ first_arg : IsZero , $ ($ rest : IsZero ,) *> IsZero for ($ first_arg , $ ($ rest ,) *) { # [inline] fn is_zero (& self) -> bool { # [allow (non_snake_case)] let ($ first_arg , $ ($ rest ,) *) = self ; $ first_arg . is_zero () $ (&& $ rest . is_zero ()) * } } impl_is_zero_tuples ! ($ ($ rest) ,*) ; } }}
mkitem!{impl_is_zero_tuples ! (A , B , C , D , E , F , G , H) ;}
mkitem!{mkimpl!{unsafe impl < T : ? Sized > IsZero for Option < & T > { # [inline] fn is_zero (& self) -> bool { self . is_none () } }}}
mkitem!{mkimpl!{unsafe impl < T : ? Sized > IsZero for Option < Box < T > > { # [inline] fn is_zero (& self) -> bool { self . is_none () } }}}
mkitem!{macro_rules ! impl_is_zero_option_of_nonzero_int { ($ ($ t : ty) ,+ $ (,) ?) => { $ (unsafe impl IsZero for Option < NonZero <$ t >> { # [inline] fn is_zero (& self) -> bool { self . is_none () } }) + } ; }}
mkitem!{impl_is_zero_option_of_nonzero_int ! (u8 , u16 , u32 , u64 , u128 , usize , i8 , i16 , i32 , i64 , i128 , isize) ;}
mkitem!{macro_rules ! impl_is_zero_option_of_int { ($ ($ t : ty) ,+ $ (,) ?) => { $ (unsafe impl IsZero for Option <$ t > { # [inline] fn is_zero (& self) -> bool { const { let none : Self = unsafe { core :: mem :: MaybeUninit :: zeroed () . assume_init () } ; assert ! (none . is_none ()) ; } self . is_none () } }) + } ; }}
mkitem!{impl_is_zero_option_of_int ! (u8 , u16 , u32 , u64 , u128 , i8 , i16 , i32 , i64 , i128 , usize , isize) ;}
mkitem!{mkimpl!{unsafe impl < T : IsZero > IsZero for Wrapping < T > { # [inline] fn is_zero (& self) -> bool { self . 0 . is_zero () } }}}
mkitem!{mkimpl!{unsafe impl < T : IsZero > IsZero for Saturating < T > { # [inline] fn is_zero (& self) -> bool { self . 0 . is_zero () } }}}
mkitem!{macro_rules ! impl_is_zero_option_of_bool { ($ ($ t : ty) ,+ $ (,) ?) => { $ (unsafe impl IsZero for $ t { # [inline] fn is_zero (& self) -> bool { let raw : u8 = unsafe { core :: mem :: transmute (* self) } ; raw == 0 } }) + } ; }}
mkitem!{impl_is_zero_option_of_bool ! { Option < bool >, Option < Option < bool >>, Option < Option < Option < bool >>>, }}