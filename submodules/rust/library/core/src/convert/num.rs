mkuse!{use crate :: num :: TryFromIntError ;}
mkmod!{private, { 
                getname!(private);
                getsrc!(private);
                getpath!(private);
                get_deps!(private);
                get_crates!(private);
                mkinclude!(private);
                mkitem!{mktrait!{# [doc = " This trait being unreachable from outside the crate"] # [doc = " prevents other implementations of the `FloatToInt` trait,"] # [doc = " which allows potentially adding more trait methods after the trait is `#[stable]`."] # [unstable (feature = "convert_float_to_int" , issue = "67057")] pub trait Sealed { }}} 
            }}
mkitem!{mktrait!{# [doc = " Supporting trait for inherent methods of `f32` and `f64` such as `to_int_unchecked`."] # [doc = " Typically doesn’t need to be used directly."] # [unstable (feature = "convert_float_to_int" , issue = "67057")] pub trait FloatToInt < Int > : private :: Sealed + Sized { # [unstable (feature = "convert_float_to_int" , issue = "67057")] # [doc (hidden)] unsafe fn to_int_unchecked (self) -> Int ; }}}
mkitem!{macro_rules ! impl_float_to_int { ($ Float : ty => $ ($ Int : ty) ,+) => { # [unstable (feature = "convert_float_to_int" , issue = "67057")] impl private :: Sealed for $ Float { } $ (# [unstable (feature = "convert_float_to_int" , issue = "67057")] impl FloatToInt <$ Int > for $ Float { # [inline] unsafe fn to_int_unchecked (self) -> $ Int { unsafe { crate :: intrinsics :: float_to_int_unchecked (self) } } }) + } }}
mkitem!{impl_float_to_int ! (f16 => u8 , u16 , u32 , u64 , u128 , usize , i8 , i16 , i32 , i64 , i128 , isize) ;}
mkitem!{impl_float_to_int ! (f32 => u8 , u16 , u32 , u64 , u128 , usize , i8 , i16 , i32 , i64 , i128 , isize) ;}
mkitem!{impl_float_to_int ! (f64 => u8 , u16 , u32 , u64 , u128 , usize , i8 , i16 , i32 , i64 , i128 , isize) ;}
mkitem!{impl_float_to_int ! (f128 => u8 , u16 , u32 , u64 , u128 , usize , i8 , i16 , i32 , i64 , i128 , isize) ;}
mkitem!{macro_rules ! impl_from { (bool => $ Int : ty $ (,) ?) => { impl_from ! (bool => $ Int , # [stable (feature = "from_bool" , since = "1.28.0")] , concat ! ("Converts a [`bool`] to [`" , stringify ! ($ Int) , "`] losslessly.\n" , "The resulting value is `0` for `false` and `1` for `true` values.\n" , "\n" , "# Examples\n" , "\n" , "```\n" , "assert_eq!(" , stringify ! ($ Int) , "::from(true), 1);\n" , "assert_eq!(" , stringify ! ($ Int) , "::from(false), 0);\n" , "```\n" ,) ,) ; } ; ($ Small : ty => $ Large : ty , # [$ attr : meta] $ (,) ?) => { impl_from ! ($ Small => $ Large , # [$ attr] , concat ! ("Converts [`" , stringify ! ($ Small) , "`] to [`" , stringify ! ($ Large) , "`] losslessly.") ,) ; } ; ($ Small : ty => $ Large : ty , # [$ attr : meta] , $ doc : expr $ (,) ?) => { # [$ attr] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const From <$ Small > for $ Large { # [doc = $ doc] # [inline (always)] fn from (small : $ Small) -> Self { small as Self } } } ; }}
mkitem!{impl_from ! (bool => u8) ;}
mkitem!{impl_from ! (bool => u16) ;}
mkitem!{impl_from ! (bool => u32) ;}
mkitem!{impl_from ! (bool => u64) ;}
mkitem!{impl_from ! (bool => u128) ;}
mkitem!{impl_from ! (bool => usize) ;}
mkitem!{impl_from ! (bool => i8) ;}
mkitem!{impl_from ! (bool => i16) ;}
mkitem!{impl_from ! (bool => i32) ;}
mkitem!{impl_from ! (bool => i64) ;}
mkitem!{impl_from ! (bool => i128) ;}
mkitem!{impl_from ! (bool => isize) ;}
mkitem!{impl_from ! (u8 => u16 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u8 => u32 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u8 => u64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u8 => u128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u8 => usize , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u16 => u32 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u16 => u64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u16 => u128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u32 => u64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u32 => u128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u64 => u128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (i8 => i16 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (i8 => i32 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (i8 => i64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (i8 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (i8 => isize , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (i16 => i32 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (i16 => i64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (i16 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (i32 => i64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (i32 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (i64 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u8 => i16 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u8 => i32 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u8 => i64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u8 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u16 => i32 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u16 => i64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u16 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u32 => i64 , # [stable (feature = "lossless_int_conv" , since = "1.5.0")]) ;}
mkitem!{impl_from ! (u32 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u64 => i128 , # [stable (feature = "i128" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u16 => usize , # [stable (feature = "lossless_iusize_conv" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (u8 => isize , # [stable (feature = "lossless_iusize_conv" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (i16 => isize , # [stable (feature = "lossless_iusize_conv" , since = "1.26.0")]) ;}
mkitem!{impl_from ! (i8 => f16 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i8 => f32 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i8 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i8 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i16 => f32 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i16 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i16 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i32 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (i32 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u8 => f16 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u8 => f32 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u8 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u8 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u16 => f32 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u16 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u16 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u32 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (u32 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (f16 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (f16 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (f32 => f64 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (f32 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{impl_from ! (f64 => f128 , # [stable (feature = "lossless_float_conv" , since = "1.6.0")]) ;}
mkitem!{macro_rules ! impl_float_from_bool { ($ float : ty $ (; doctest_prefix : $ (# [doc = $ doctest_prefix : literal]) * doctest_suffix : $ (# [doc = $ doctest_suffix : literal]) *) ?) => { # [stable (feature = "float_from_bool" , since = "1.68.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const From < bool > for $ float { # [doc = concat ! ("Converts a [`bool`] to [`" , stringify ! ($ float) , "`] losslessly.")] # [doc = " The resulting value is positive `0.0` for `false` and `1.0` for `true` values."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] $ ($ (# [doc = $ doctest_prefix]) *) ? # [doc = concat ! ("let x: " , stringify ! ($ float) , " = false.into();")] # [doc = " assert_eq!(x, 0.0);"] # [doc = " assert!(x.is_sign_positive());"] # [doc = ""] # [doc = concat ! ("let y: " , stringify ! ($ float) , " = true.into();")] # [doc = " assert_eq!(y, 1.0);"] $ ($ (# [doc = $ doctest_suffix]) *) ? # [doc = " ```"] # [inline] fn from (small : bool) -> Self { small as u8 as Self } } } ; }}
mkitem!{impl_float_from_bool ! (f16 ; doctest_prefix : # [doc = "#![feature(f16)]"] # [doc = "# #[cfg(all(target_arch = \"x86_64\", target_os = \"linux\"))] {"] # [doc = ""] doctest_suffix : # [doc = "# }"]) ;}
mkitem!{impl_float_from_bool ! (f32) ;}
mkitem!{impl_float_from_bool ! (f64) ;}
mkitem!{impl_float_from_bool ! (f128 ; doctest_prefix : # [doc = "#![feature(f128)]"] # [doc = "# #[cfg(all(target_arch = \"x86_64\", target_os = \"linux\"))] {"] # [doc = ""] doctest_suffix : # [doc = "# }"]) ;}
mkitem!{macro_rules ! impl_try_from_unbounded { ($ source : ty => $ ($ target : ty) ,+) => { $ (# [stable (feature = "try_from" , since = "1.34.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const TryFrom <$ source > for $ target { type Error = TryFromIntError ; # [doc = " Tries to create the target number type from a source"] # [doc = " number type. This returns an error if the source value"] # [doc = " is outside of the range of the target type."] # [inline] fn try_from (value : $ source) -> Result < Self , Self :: Error > { Ok (value as Self) } }) * } }}
mkitem!{macro_rules ! impl_try_from_lower_bounded { ($ source : ty => $ ($ target : ty) ,+) => { $ (# [stable (feature = "try_from" , since = "1.34.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const TryFrom <$ source > for $ target { type Error = TryFromIntError ; # [doc = " Tries to create the target number type from a source"] # [doc = " number type. This returns an error if the source value"] # [doc = " is outside of the range of the target type."] # [inline] fn try_from (u : $ source) -> Result < Self , Self :: Error > { if u >= 0 { Ok (u as Self) } else { Err (TryFromIntError (())) } } }) * } }}
mkitem!{macro_rules ! impl_try_from_upper_bounded { ($ source : ty => $ ($ target : ty) ,+) => { $ (# [stable (feature = "try_from" , since = "1.34.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const TryFrom <$ source > for $ target { type Error = TryFromIntError ; # [doc = " Tries to create the target number type from a source"] # [doc = " number type. This returns an error if the source value"] # [doc = " is outside of the range of the target type."] # [inline] fn try_from (u : $ source) -> Result < Self , Self :: Error > { if u > (Self :: MAX as $ source) { Err (TryFromIntError (())) } else { Ok (u as Self) } } }) * } }}
mkitem!{macro_rules ! impl_try_from_both_bounded { ($ source : ty => $ ($ target : ty) ,+) => { $ (# [stable (feature = "try_from" , since = "1.34.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const TryFrom <$ source > for $ target { type Error = TryFromIntError ; # [doc = " Tries to create the target number type from a source"] # [doc = " number type. This returns an error if the source value"] # [doc = " is outside of the range of the target type."] # [inline] fn try_from (u : $ source) -> Result < Self , Self :: Error > { let min = Self :: MIN as $ source ; let max = Self :: MAX as $ source ; if u < min || u > max { Err (TryFromIntError (())) } else { Ok (u as Self) } } }) * } }}
mkitem!{macro_rules ! rev { ($ mac : ident , $ source : ty => $ ($ target : ty) ,+) => { $ ($ mac ! ($ target => $ source) ;) * } }}
mkitem!{impl_try_from_upper_bounded ! (u16 => u8) ;}
mkitem!{impl_try_from_upper_bounded ! (u32 => u8 , u16) ;}
mkitem!{impl_try_from_upper_bounded ! (u64 => u8 , u16 , u32) ;}
mkitem!{impl_try_from_upper_bounded ! (u128 => u8 , u16 , u32 , u64) ;}
mkitem!{impl_try_from_both_bounded ! (i16 => i8) ;}
mkitem!{impl_try_from_both_bounded ! (i32 => i8 , i16) ;}
mkitem!{impl_try_from_both_bounded ! (i64 => i8 , i16 , i32) ;}
mkitem!{impl_try_from_both_bounded ! (i128 => i8 , i16 , i32 , i64) ;}
mkitem!{impl_try_from_upper_bounded ! (u8 => i8) ;}
mkitem!{impl_try_from_upper_bounded ! (u16 => i8 , i16) ;}
mkitem!{impl_try_from_upper_bounded ! (u32 => i8 , i16 , i32) ;}
mkitem!{impl_try_from_upper_bounded ! (u64 => i8 , i16 , i32 , i64) ;}
mkitem!{impl_try_from_upper_bounded ! (u128 => i8 , i16 , i32 , i64 , i128) ;}
mkitem!{impl_try_from_lower_bounded ! (i8 => u8 , u16 , u32 , u64 , u128) ;}
mkitem!{impl_try_from_both_bounded ! (i16 => u8) ;}
mkitem!{impl_try_from_lower_bounded ! (i16 => u16 , u32 , u64 , u128) ;}
mkitem!{impl_try_from_both_bounded ! (i32 => u8 , u16) ;}
mkitem!{impl_try_from_lower_bounded ! (i32 => u32 , u64 , u128) ;}
mkitem!{impl_try_from_both_bounded ! (i64 => u8 , u16 , u32) ;}
mkitem!{impl_try_from_lower_bounded ! (i64 => u64 , u128) ;}
mkitem!{impl_try_from_both_bounded ! (i128 => u8 , u16 , u32 , u64) ;}
mkitem!{impl_try_from_lower_bounded ! (i128 => u128) ;}
mkitem!{impl_try_from_upper_bounded ! (usize => isize) ;}
mkitem!{impl_try_from_lower_bounded ! (isize => usize) ;}
mkmod!{ptr_try_from_impls, { 
                getname!(ptr_try_from_impls);
                getsrc!(ptr_try_from_impls);
                getpath!(ptr_try_from_impls);
                get_deps!(ptr_try_from_impls);
                get_crates!(ptr_try_from_impls);
                mkinclude!(ptr_try_from_impls);
                mkuse!{use super :: TryFromIntError ;}
mkitem!{impl_try_from_upper_bounded ! (usize => u8) ;}
mkitem!{impl_try_from_unbounded ! (usize => u16 , u32 , u64 , u128) ;}
mkitem!{impl_try_from_upper_bounded ! (usize => i8 , i16) ;}
mkitem!{impl_try_from_unbounded ! (usize => i32 , i64 , i128) ;}
mkitem!{impl_try_from_both_bounded ! (isize => u8) ;}
mkitem!{impl_try_from_lower_bounded ! (isize => u16 , u32 , u64 , u128) ;}
mkitem!{impl_try_from_both_bounded ! (isize => i8) ;}
mkitem!{impl_try_from_unbounded ! (isize => i16 , i32 , i64 , i128) ;}
mkitem!{rev ! (impl_try_from_upper_bounded , usize => u32 , u64 , u128) ;}
mkitem!{rev ! (impl_try_from_lower_bounded , usize => i8 , i16) ;}
mkitem!{rev ! (impl_try_from_both_bounded , usize => i32 , i64 , i128) ;}
mkitem!{rev ! (impl_try_from_upper_bounded , isize => u16 , u32 , u64 , u128) ;}
mkitem!{rev ! (impl_try_from_both_bounded , isize => i32 , i64 , i128) ;} 
            }}
mkmod!{ptr_try_from_impls, { 
                getname!(ptr_try_from_impls);
                getsrc!(ptr_try_from_impls);
                getpath!(ptr_try_from_impls);
                get_deps!(ptr_try_from_impls);
                get_crates!(ptr_try_from_impls);
                mkinclude!(ptr_try_from_impls);
                mkuse!{use super :: TryFromIntError ;}
mkitem!{impl_try_from_upper_bounded ! (usize => u8 , u16) ;}
mkitem!{impl_try_from_unbounded ! (usize => u32 , u64 , u128) ;}
mkitem!{impl_try_from_upper_bounded ! (usize => i8 , i16 , i32) ;}
mkitem!{impl_try_from_unbounded ! (usize => i64 , i128) ;}
mkitem!{impl_try_from_both_bounded ! (isize => u8 , u16) ;}
mkitem!{impl_try_from_lower_bounded ! (isize => u32 , u64 , u128) ;}
mkitem!{impl_try_from_both_bounded ! (isize => i8 , i16) ;}
mkitem!{impl_try_from_unbounded ! (isize => i32 , i64 , i128) ;}
mkitem!{rev ! (impl_try_from_unbounded , usize => u32) ;}
mkitem!{rev ! (impl_try_from_upper_bounded , usize => u64 , u128) ;}
mkitem!{rev ! (impl_try_from_lower_bounded , usize => i8 , i16 , i32) ;}
mkitem!{rev ! (impl_try_from_both_bounded , usize => i64 , i128) ;}
mkitem!{rev ! (impl_try_from_unbounded , isize => u16) ;}
mkitem!{rev ! (impl_try_from_upper_bounded , isize => u32 , u64 , u128) ;}
mkitem!{rev ! (impl_try_from_unbounded , isize => i32) ;}
mkitem!{rev ! (impl_try_from_both_bounded , isize => i64 , i128) ;} 
            }}
mkmod!{ptr_try_from_impls, { 
                getname!(ptr_try_from_impls);
                getsrc!(ptr_try_from_impls);
                getpath!(ptr_try_from_impls);
                get_deps!(ptr_try_from_impls);
                get_crates!(ptr_try_from_impls);
                mkinclude!(ptr_try_from_impls);
                mkuse!{use super :: TryFromIntError ;}
mkitem!{impl_try_from_upper_bounded ! (usize => u8 , u16 , u32) ;}
mkitem!{impl_try_from_unbounded ! (usize => u64 , u128) ;}
mkitem!{impl_try_from_upper_bounded ! (usize => i8 , i16 , i32 , i64) ;}
mkitem!{impl_try_from_unbounded ! (usize => i128) ;}
mkitem!{impl_try_from_both_bounded ! (isize => u8 , u16 , u32) ;}
mkitem!{impl_try_from_lower_bounded ! (isize => u64 , u128) ;}
mkitem!{impl_try_from_both_bounded ! (isize => i8 , i16 , i32) ;}
mkitem!{impl_try_from_unbounded ! (isize => i64 , i128) ;}
mkitem!{rev ! (impl_try_from_unbounded , usize => u32 , u64) ;}
mkitem!{rev ! (impl_try_from_upper_bounded , usize => u128) ;}
mkitem!{rev ! (impl_try_from_lower_bounded , usize => i8 , i16 , i32 , i64) ;}
mkitem!{rev ! (impl_try_from_both_bounded , usize => i128) ;}
mkitem!{rev ! (impl_try_from_unbounded , isize => u16 , u32) ;}
mkitem!{rev ! (impl_try_from_upper_bounded , isize => u64 , u128) ;}
mkitem!{rev ! (impl_try_from_unbounded , isize => i32 , i64) ;}
mkitem!{rev ! (impl_try_from_both_bounded , isize => i128) ;} 
            }}
mkuse!{use crate :: num :: NonZero ;}
mkitem!{macro_rules ! impl_nonzero_int_from_nonzero_int { ($ Small : ty => $ Large : ty) => { # [stable (feature = "nz_int_conv" , since = "1.41.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const From < NonZero <$ Small >> for NonZero <$ Large > { # [doc = concat ! ("Converts <code>[NonZero]\\<[" , stringify ! ($ Small) , "]></code> ")] # [doc = concat ! ("to <code>[NonZero]\\<[" , stringify ! ($ Large) , "]></code> losslessly.")] # [inline] fn from (small : NonZero <$ Small >) -> Self { unsafe { Self :: new_unchecked (From :: from (small . get ())) } } } } ; }}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => u16) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => u32) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => u64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => u128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => usize) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u16 => u32) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u16 => u64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u16 => u128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u16 => usize) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u32 => u64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u32 => u128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u64 => u128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i8 => i16) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i8 => i32) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i8 => i64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i8 => i128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i8 => isize) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i16 => i32) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i16 => i64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i16 => i128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i16 => isize) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i32 => i64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i32 => i128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (i64 => i128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => i16) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => i32) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => i64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => i128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u8 => isize) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u16 => i32) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u16 => i64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u16 => i128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u32 => i64) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u32 => i128) ;}
mkitem!{impl_nonzero_int_from_nonzero_int ! (u64 => i128) ;}
mkitem!{macro_rules ! impl_nonzero_int_try_from_int { ($ Int : ty) => { # [stable (feature = "nzint_try_from_int_conv" , since = "1.46.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const TryFrom <$ Int > for NonZero <$ Int > { type Error = TryFromIntError ; # [doc = concat ! ("Attempts to convert [`" , stringify ! ($ Int) , "`] ")] # [doc = concat ! ("to <code>[NonZero]\\<[" , stringify ! ($ Int) , "]></code>.")] # [inline] fn try_from (value : $ Int) -> Result < Self , Self :: Error > { Self :: new (value) . ok_or (TryFromIntError (())) } } } ; }}
mkitem!{impl_nonzero_int_try_from_int ! (u8) ;}
mkitem!{impl_nonzero_int_try_from_int ! (u16) ;}
mkitem!{impl_nonzero_int_try_from_int ! (u32) ;}
mkitem!{impl_nonzero_int_try_from_int ! (u64) ;}
mkitem!{impl_nonzero_int_try_from_int ! (u128) ;}
mkitem!{impl_nonzero_int_try_from_int ! (usize) ;}
mkitem!{impl_nonzero_int_try_from_int ! (i8) ;}
mkitem!{impl_nonzero_int_try_from_int ! (i16) ;}
mkitem!{impl_nonzero_int_try_from_int ! (i32) ;}
mkitem!{impl_nonzero_int_try_from_int ! (i64) ;}
mkitem!{impl_nonzero_int_try_from_int ! (i128) ;}
mkitem!{impl_nonzero_int_try_from_int ! (isize) ;}
mkitem!{macro_rules ! impl_nonzero_int_try_from_nonzero_int { ($ source : ty => $ ($ target : ty) ,+) => { $ (# [stable (feature = "nzint_try_from_nzint_conv" , since = "1.49.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const TryFrom < NonZero <$ source >> for NonZero <$ target > { type Error = TryFromIntError ; # [doc = concat ! ("Attempts to convert <code>[NonZero]\\<[" , stringify ! ($ source) , "]></code> ")] # [doc = concat ! ("to <code>[NonZero]\\<[" , stringify ! ($ target) , "]></code>.")] # [inline] fn try_from (value : NonZero <$ source >) -> Result < Self , Self :: Error > { Ok (unsafe { Self :: new_unchecked (<$ target >:: try_from (value . get ()) ?) }) } }) * } ; }}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u16 => u8) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u32 => u8 , u16 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u64 => u8 , u16 , u32 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u128 => u8 , u16 , u32 , u64 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (usize => u8 , u16 , u32 , u64 , u128) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i16 => i8) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i32 => i8 , i16 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i64 => i8 , i16 , i32 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i128 => i8 , i16 , i32 , i64 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (isize => i8 , i16 , i32 , i64 , i128) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u8 => i8) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u16 => i8 , i16 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u32 => i8 , i16 , i32 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u64 => i8 , i16 , i32 , i64 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (u128 => i8 , i16 , i32 , i64 , i128 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (usize => i8 , i16 , i32 , i64 , i128 , isize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i8 => u8 , u16 , u32 , u64 , u128 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i16 => u8 , u16 , u32 , u64 , u128 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i32 => u8 , u16 , u32 , u64 , u128 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i64 => u8 , u16 , u32 , u64 , u128 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (i128 => u8 , u16 , u32 , u64 , u128 , usize) ;}
mkitem!{impl_nonzero_int_try_from_nonzero_int ! (isize => u8 , u16 , u32 , u64 , u128 , usize) ;}