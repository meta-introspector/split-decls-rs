/* FP:mini_core.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0001
/* FP:mini_core.rs-0002 */ # [feature (no_core , lang_items , intrinsics , unboxed_closures , extern_types , decl_macro , rustc_attrs , transparent_unions , auto_traits , freeze_impls , thread_local)] # [no_core] # [allow (dead_code , internal_features , ambiguous_wide_pointer_comparisons)] # [unsafe (no_mangle)] unsafe extern "C" fn _Unwind_Resume () { intrinsics :: unreachable () ; }
/* FP:mini_core.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0002
/* FP:mini_core.rs-0004 */ # [lang = "pointee_sized"] pub trait PointeeSized { }
/* FP:mini_core.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0003
/* FP:mini_core.rs-0006 */ # [lang = "meta_sized"] pub trait MetaSized : PointeeSized { }
/* FP:mini_core.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0004
/* FP:mini_core.rs-0008 */ # [lang = "sized"] pub trait Sized : MetaSized { }
/* FP:mini_core.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0005
/* FP:mini_core.rs-0010 */ # [lang = "destruct"] pub trait Destruct { }
/* FP:mini_core.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0006
/* FP:mini_core.rs-0012 */ # [lang = "tuple_trait"] pub trait Tuple { }
/* FP:mini_core.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0007
/* FP:mini_core.rs-0014 */ # [lang = "unsize"] pub trait Unsize < T : PointeeSized > : PointeeSized { }
/* FP:mini_core.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0008
/* FP:mini_core.rs-0016 */ # [lang = "coerce_unsized"] pub trait CoerceUnsized < T > { }
/* FP:mini_core.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0009
/* FP:mini_core.rs-0018 */ impl < 'a , 'b : 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < & 'a U > for & 'b T { }
/* FP:mini_core.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0010
/* FP:mini_core.rs-0020 */ impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < & 'a mut U > for & 'a mut T { }
/* FP:mini_core.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0011
/* FP:mini_core.rs-0022 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < * const U > for * const T { }
/* FP:mini_core.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0012
/* FP:mini_core.rs-0024 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < * mut U > for * mut T { }
/* FP:mini_core.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0013
/* FP:mini_core.rs-0026 */ # [lang = "dispatch_from_dyn"] pub trait DispatchFromDyn < T > { }
/* FP:mini_core.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0014
/* FP:mini_core.rs-0028 */ impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < & 'a U > for & 'a T { }
/* FP:mini_core.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0015
/* FP:mini_core.rs-0030 */ impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < & 'a mut U > for & 'a mut T { }
/* FP:mini_core.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0016
/* FP:mini_core.rs-0032 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < * const U > for * const T { }
/* FP:mini_core.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0017
/* FP:mini_core.rs-0034 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < * mut U > for * mut T { }
/* FP:mini_core.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0018
/* FP:mini_core.rs-0036 */ impl < T : MetaSized + Unsize < U > , U : MetaSized > DispatchFromDyn < Box < U , () > > for Box < T , () > { }
/* FP:mini_core.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0019
/* FP:mini_core.rs-0038 */ # [lang = "legacy_receiver"] pub trait LegacyReceiver { }
/* FP:mini_core.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0020
/* FP:mini_core.rs-0040 */ impl < T : PointeeSized > LegacyReceiver for & T { }
/* FP:mini_core.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0021
/* FP:mini_core.rs-0042 */ impl < T : PointeeSized > LegacyReceiver for & mut T { }
/* FP:mini_core.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0022
/* FP:mini_core.rs-0044 */ impl < T : MetaSized > LegacyReceiver for Box < T > { }
/* FP:mini_core.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0023
/* FP:mini_core.rs-0046 */ # [lang = "receiver"] trait Receiver { }
/* FP:mini_core.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0024
/* FP:mini_core.rs-0048 */ # [lang = "copy"] pub trait Copy { }
/* FP:mini_core.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0025
/* FP:mini_core.rs-0050 */ # [lang = "bikeshed_guaranteed_no_drop"] pub trait BikeshedGuaranteedNoDrop { }
/* FP:mini_core.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0026
/* FP:mini_core.rs-0052 */ impl Copy for bool { }
/* FP:mini_core.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0027
/* FP:mini_core.rs-0054 */ impl Copy for u8 { }
/* FP:mini_core.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0028
/* FP:mini_core.rs-0056 */ impl Copy for u16 { }
/* FP:mini_core.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0029
/* FP:mini_core.rs-0058 */ impl Copy for u32 { }
/* FP:mini_core.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0030
/* FP:mini_core.rs-0060 */ impl Copy for u64 { }
/* FP:mini_core.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0031
/* FP:mini_core.rs-0062 */ impl Copy for usize { }
/* FP:mini_core.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0032
/* FP:mini_core.rs-0064 */ impl Copy for u128 { }
/* FP:mini_core.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0033
/* FP:mini_core.rs-0066 */ impl Copy for i8 { }
/* FP:mini_core.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0034
/* FP:mini_core.rs-0068 */ impl Copy for i16 { }
/* FP:mini_core.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0035
/* FP:mini_core.rs-0070 */ impl Copy for i32 { }
/* FP:mini_core.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0036
/* FP:mini_core.rs-0072 */ impl Copy for i64 { }
/* FP:mini_core.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0037
/* FP:mini_core.rs-0074 */ impl Copy for isize { }
/* FP:mini_core.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0038
/* FP:mini_core.rs-0076 */ impl Copy for i128 { }
/* FP:mini_core.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0039
/* FP:mini_core.rs-0078 */ impl Copy for f32 { }
/* FP:mini_core.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0040
/* FP:mini_core.rs-0080 */ impl Copy for f64 { }
/* FP:mini_core.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0041
/* FP:mini_core.rs-0082 */ impl Copy for char { }
/* FP:mini_core.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0042
/* FP:mini_core.rs-0084 */ impl < 'a , T : PointeeSized > Copy for & 'a T { }
/* FP:mini_core.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0043
/* FP:mini_core.rs-0086 */ impl < T : PointeeSized > Copy for * const T { }
/* FP:mini_core.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0044
/* FP:mini_core.rs-0088 */ impl < T : PointeeSized > Copy for * mut T { }
/* FP:mini_core.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0045
/* FP:mini_core.rs-0090 */ # [lang = "sync"] pub unsafe trait Sync { }
/* FP:mini_core.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0046
/* FP:mini_core.rs-0092 */ unsafe impl Sync for bool { }
/* FP:mini_core.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0047
/* FP:mini_core.rs-0094 */ unsafe impl Sync for u8 { }
/* FP:mini_core.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0048
/* FP:mini_core.rs-0096 */ unsafe impl Sync for u16 { }
/* FP:mini_core.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0049
/* FP:mini_core.rs-0098 */ unsafe impl Sync for u32 { }
/* FP:mini_core.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0050
/* FP:mini_core.rs-0100 */ unsafe impl Sync for u64 { }
/* FP:mini_core.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0051
/* FP:mini_core.rs-0102 */ unsafe impl Sync for usize { }
/* FP:mini_core.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0052
/* FP:mini_core.rs-0104 */ unsafe impl Sync for i8 { }
/* FP:mini_core.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0053
/* FP:mini_core.rs-0106 */ unsafe impl Sync for i16 { }
/* FP:mini_core.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0054
/* FP:mini_core.rs-0108 */ unsafe impl Sync for i32 { }
/* FP:mini_core.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0055
/* FP:mini_core.rs-0110 */ unsafe impl Sync for isize { }
/* FP:mini_core.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0056
/* FP:mini_core.rs-0112 */ unsafe impl Sync for char { }
/* FP:mini_core.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0057
/* FP:mini_core.rs-0114 */ unsafe impl < 'a , T : PointeeSized > Sync for & 'a T { }
/* FP:mini_core.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0058
/* FP:mini_core.rs-0116 */ unsafe impl Sync for [u8 ; 16] { }
/* FP:mini_core.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0059
/* FP:mini_core.rs-0118 */ # [lang = "freeze"] unsafe auto trait Freeze { }
/* FP:mini_core.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0060
/* FP:mini_core.rs-0120 */ unsafe impl < T : PointeeSized > Freeze for PhantomData < T > { }
/* FP:mini_core.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0061
/* FP:mini_core.rs-0122 */ unsafe impl < T : PointeeSized > Freeze for * const T { }
/* FP:mini_core.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0062
/* FP:mini_core.rs-0124 */ unsafe impl < T : PointeeSized > Freeze for * mut T { }
/* FP:mini_core.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0063
/* FP:mini_core.rs-0126 */ unsafe impl < T : PointeeSized > Freeze for & T { }
/* FP:mini_core.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0064
/* FP:mini_core.rs-0128 */ unsafe impl < T : PointeeSized > Freeze for & mut T { }
/* FP:mini_core.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0065
/* FP:mini_core.rs-0130 */ # [lang = "structural_peq"] pub trait StructuralPartialEq { }
/* FP:mini_core.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0066
/* FP:mini_core.rs-0132 */ # [lang = "not"] pub trait Not { type Output ; fn not (self) -> Self :: Output ; }
/* FP:mini_core.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0067
/* FP:mini_core.rs-0134 */ impl Not for bool { type Output = bool ; fn not (self) -> bool { ! self } }
/* FP:mini_core.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0068
/* FP:mini_core.rs-0136 */ # [lang = "mul"] pub trait Mul < RHS = Self > { type Output ; # [must_use] fn mul (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0069
/* FP:mini_core.rs-0138 */ impl Mul for u8 { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }
/* FP:mini_core.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0070
/* FP:mini_core.rs-0140 */ impl Mul for i32 { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }
/* FP:mini_core.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0071
/* FP:mini_core.rs-0142 */ impl Mul for usize { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }
/* FP:mini_core.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0072
/* FP:mini_core.rs-0144 */ impl Mul for isize { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }
/* FP:mini_core.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0073
/* FP:mini_core.rs-0146 */ # [lang = "add"] pub trait Add < RHS = Self > { type Output ; fn add (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0074
/* FP:mini_core.rs-0148 */ impl Add for u8 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0075
/* FP:mini_core.rs-0150 */ impl Add for i8 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0076
/* FP:mini_core.rs-0152 */ impl Add for i32 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0077
/* FP:mini_core.rs-0154 */ impl Add for usize { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0078
/* FP:mini_core.rs-0156 */ impl Add for isize { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0079
/* FP:mini_core.rs-0158 */ # [lang = "sub"] pub trait Sub < RHS = Self > { type Output ; fn sub (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0080
/* FP:mini_core.rs-0160 */ impl Sub for usize { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0081
/* FP:mini_core.rs-0162 */ impl Sub for isize { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0082
/* FP:mini_core.rs-0164 */ impl Sub for u8 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0083
/* FP:mini_core.rs-0166 */ impl Sub for i8 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0084
/* FP:mini_core.rs-0168 */ impl Sub for i16 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0085
/* FP:mini_core.rs-0170 */ impl Sub for i32 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0086
/* FP:mini_core.rs-0172 */ # [lang = "rem"] pub trait Rem < RHS = Self > { type Output ; fn rem (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0087
/* FP:mini_core.rs-0174 */ impl Rem for usize { type Output = Self ; fn rem (self , rhs : Self) -> Self { self % rhs } }
/* FP:mini_core.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0088
/* FP:mini_core.rs-0176 */ # [lang = "bitor"] pub trait BitOr < RHS = Self > { type Output ; # [must_use] fn bitor (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0177 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0089
/* FP:mini_core.rs-0178 */ impl BitOr for bool { type Output = bool ; fn bitor (self , rhs : bool) -> bool { self | rhs } }
/* FP:mini_core.rs-0179 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0090
/* FP:mini_core.rs-0180 */ impl < 'a > BitOr < bool > for & 'a bool { type Output = bool ; fn bitor (self , rhs : bool) -> bool { * self | rhs } }
/* FP:mini_core.rs-0181 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0091
/* FP:mini_core.rs-0182 */ # [lang = "eq"] pub trait PartialEq < Rhs : ? Sized = Self > { fn eq (& self , other : & Rhs) -> bool ; fn ne (& self , other : & Rhs) -> bool ; }
/* FP:mini_core.rs-0183 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0092
/* FP:mini_core.rs-0184 */ impl PartialEq for u8 { fn eq (& self , other : & u8) -> bool { (* self) == (* other) } fn ne (& self , other : & u8) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0185 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0093
/* FP:mini_core.rs-0186 */ impl PartialEq for u16 { fn eq (& self , other : & u16) -> bool { (* self) == (* other) } fn ne (& self , other : & u16) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0187 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0094
/* FP:mini_core.rs-0188 */ impl PartialEq for u32 { fn eq (& self , other : & u32) -> bool { (* self) == (* other) } fn ne (& self , other : & u32) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0189 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0095
/* FP:mini_core.rs-0190 */ impl PartialEq for u64 { fn eq (& self , other : & u64) -> bool { (* self) == (* other) } fn ne (& self , other : & u64) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0191 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0096
/* FP:mini_core.rs-0192 */ impl PartialEq for usize { fn eq (& self , other : & usize) -> bool { (* self) == (* other) } fn ne (& self , other : & usize) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0193 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0097
/* FP:mini_core.rs-0194 */ impl PartialEq for i8 { fn eq (& self , other : & i8) -> bool { (* self) == (* other) } fn ne (& self , other : & i8) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0195 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0098
/* FP:mini_core.rs-0196 */ impl PartialEq for i32 { fn eq (& self , other : & i32) -> bool { (* self) == (* other) } fn ne (& self , other : & i32) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0197 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0099
/* FP:mini_core.rs-0198 */ impl PartialEq for isize { fn eq (& self , other : & isize) -> bool { (* self) == (* other) } fn ne (& self , other : & isize) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0199 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0100
/* FP:mini_core.rs-0200 */ impl PartialEq for char { fn eq (& self , other : & char) -> bool { (* self) == (* other) } fn ne (& self , other : & char) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0201 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0101
/* FP:mini_core.rs-0202 */ impl < T : ? Sized > PartialEq for * const T { fn eq (& self , other : & * const T) -> bool { * self == * other } fn ne (& self , other : & * const T) -> bool { * self != * other } }
/* FP:mini_core.rs-0203 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0102
/* FP:mini_core.rs-0204 */ # [lang = "neg"] pub trait Neg { type Output ; fn neg (self) -> Self :: Output ; }
/* FP:mini_core.rs-0205 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0103
/* FP:mini_core.rs-0206 */ impl Neg for i8 { type Output = i8 ; fn neg (self) -> i8 { - self } }
/* FP:mini_core.rs-0207 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0104
/* FP:mini_core.rs-0208 */ impl Neg for i16 { type Output = i16 ; fn neg (self) -> i16 { self } }
/* FP:mini_core.rs-0209 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0105
/* FP:mini_core.rs-0210 */ impl Neg for isize { type Output = isize ; fn neg (self) -> isize { - self } }
/* FP:mini_core.rs-0211 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0106
/* FP:mini_core.rs-0212 */ impl Neg for f32 { type Output = f32 ; fn neg (self) -> f32 { - self } }
/* FP:mini_core.rs-0213 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_ENUM_0107
/* FP:mini_core.rs-0214 */ pub enum Option < T > { Some (T) , None , }
/* FP:mini_core.rs-0215 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_USE_0108
/* FP:mini_core.rs-0216 */ pub use Option :: * ;
/* FP:mini_core.rs-0217 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0109
/* FP:mini_core.rs-0218 */ # [lang = "phantom_data"] pub struct PhantomData < T : PointeeSized > ;
/* FP:mini_core.rs-0219 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0110
/* FP:mini_core.rs-0220 */ # [lang = "fn_once"] # [rustc_paren_sugar] pub trait FnOnce < Args : Tuple > { # [lang = "fn_once_output"] type Output ; extern "rust-call" fn call_once (self , args : Args) -> Self :: Output ; }
/* FP:mini_core.rs-0221 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0111
/* FP:mini_core.rs-0222 */ # [lang = "fn_mut"] # [rustc_paren_sugar] pub trait FnMut < Args : Tuple > : FnOnce < Args > { extern "rust-call" fn call_mut (& mut self , args : Args) -> Self :: Output ; }
/* FP:mini_core.rs-0223 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0112
/* FP:mini_core.rs-0224 */ # [lang = "panic"] # [track_caller] pub fn panic (_msg : & 'static str) -> ! { unsafe { libc :: puts ("Panicking\n\0" as * const str as * const u8) ; intrinsics :: abort () ; } }
/* FP:mini_core.rs-0225 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_MACRO_0113
/* FP:mini_core.rs-0227 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_MACRO_0114
/* FP:mini_core.rs-0228 */ panic_const ! { panic_const_add_overflow = "attempt to add with overflow" , panic_const_sub_overflow = "attempt to subtract with overflow" , panic_const_mul_overflow = "attempt to multiply with overflow" , panic_const_div_overflow = "attempt to divide with overflow" , panic_const_rem_overflow = "attempt to calculate the remainder with overflow" , panic_const_neg_overflow = "attempt to negate with overflow" , panic_const_shr_overflow = "attempt to shift right with overflow" , panic_const_shl_overflow = "attempt to shift left with overflow" , panic_const_div_by_zero = "attempt to divide by zero" , panic_const_rem_by_zero = "attempt to calculate the remainder with a divisor of zero" , }
/* FP:mini_core.rs-0229 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0115
/* FP:mini_core.rs-0230 */ # [lang = "panic_cannot_unwind"] fn panic_cannot_unwind () -> ! { unsafe { libc :: puts ("Panicking\n\0" as * const str as * const u8) ; intrinsics :: abort () ; } }
/* FP:mini_core.rs-0231 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0116
/* FP:mini_core.rs-0232 */ # [lang = "panic_in_cleanup"] # [rustc_nounwind] fn panic_in_cleanup () -> ! { unsafe { libc :: printf ("panic in a destructor during cleanup\n\0" as * const str as * const i8) ; intrinsics :: abort () ; } }
/* FP:mini_core.rs-0233 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0117
/* FP:mini_core.rs-0234 */ # [lang = "panic_bounds_check"] # [track_caller] fn panic_bounds_check (index : usize , len : usize) -> ! { unsafe { libc :: printf ("index out of bounds: the len is %d but the index is %d\n\0" as * const str as * const i8 , len , index ,) ; intrinsics :: abort () ; } }
/* FP:mini_core.rs-0235 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0118
/* FP:mini_core.rs-0236 */ # [lang = "eh_personality"] fn eh_personality () -> ! { loop { } }
/* FP:mini_core.rs-0237 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0119
/* FP:mini_core.rs-0238 */ # [lang = "drop_in_place"] # [allow (unconditional_recursion)] pub unsafe fn drop_in_place < T : ? Sized > (to_drop : * mut T) { drop_in_place (to_drop) ; }
/* FP:mini_core.rs-0239 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0120
/* FP:mini_core.rs-0240 */ # [lang = "unpin"] pub auto trait Unpin { }
/* FP:mini_core.rs-0241 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0121
/* FP:mini_core.rs-0242 */ # [lang = "deref"] pub trait Deref { type Target : ? Sized ; fn deref (& self) -> & Self :: Target ; }
/* FP:mini_core.rs-0243 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0122
/* FP:mini_core.rs-0244 */ pub trait Allocator { }
/* FP:mini_core.rs-0245 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0123
/* FP:mini_core.rs-0246 */ impl Allocator for () { }
/* FP:mini_core.rs-0247 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0124
/* FP:mini_core.rs-0248 */ # [lang = "global_alloc_ty"] pub struct Global ;
/* FP:mini_core.rs-0249 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0125
/* FP:mini_core.rs-0250 */ impl Allocator for Global { }
/* FP:mini_core.rs-0251 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0126
/* FP:mini_core.rs-0252 */ # [repr (transparent)] # [rustc_layout_scalar_valid_range_start (1)] # [rustc_nonnull_optimization_guaranteed] pub struct NonNull < T : PointeeSized > (pub * const T) ;
/* FP:mini_core.rs-0253 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0127
/* FP:mini_core.rs-0254 */ impl < T : PointeeSized , U : PointeeSized > CoerceUnsized < NonNull < U > > for NonNull < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0255 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0128
/* FP:mini_core.rs-0256 */ impl < T : PointeeSized , U : PointeeSized > DispatchFromDyn < NonNull < U > > for NonNull < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0257 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0129
/* FP:mini_core.rs-0258 */ pub struct Unique < T : PointeeSized > { pub pointer : NonNull < T > , pub _marker : PhantomData < T > , }
/* FP:mini_core.rs-0259 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0130
/* FP:mini_core.rs-0260 */ impl < T : PointeeSized , U : PointeeSized > CoerceUnsized < Unique < U > > for Unique < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0261 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0131
/* FP:mini_core.rs-0262 */ impl < T : PointeeSized , U : PointeeSized > DispatchFromDyn < Unique < U > > for Unique < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0263 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0132
/* FP:mini_core.rs-0264 */ # [lang = "owned_box"] pub struct Box < T : ? Sized , A : Allocator = Global > (Unique < T > , A) ;
/* FP:mini_core.rs-0265 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0133
/* FP:mini_core.rs-0266 */ impl < T : ? Sized + Unsize < U > , U : ? Sized , A : Allocator > CoerceUnsized < Box < U , A > > for Box < T , A > { }
/* FP:mini_core.rs-0267 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0134
/* FP:mini_core.rs-0268 */ impl < T > Box < T > { pub fn new (val : T) -> Box < T > { unsafe { let size = intrinsics :: size_of :: < T > () ; let ptr = libc :: malloc (size) ; intrinsics :: copy (& val as * const T as * const u8 , ptr , size) ; Box (Unique { pointer : NonNull (ptr as * const T) , _marker : PhantomData } , Global) } } }
/* FP:mini_core.rs-0269 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0135
/* FP:mini_core.rs-0270 */ impl < T : ? Sized , A : Allocator > Drop for Box < T , A > { fn drop (& mut self) { unsafe { libc :: free (self . 0 . pointer . 0 as * mut u8) ; } } }
/* FP:mini_core.rs-0271 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0136
/* FP:mini_core.rs-0272 */ impl < T : ? Sized , A : Allocator > Deref for Box < T , A > { type Target = T ; fn deref (& self) -> & Self :: Target { & * * self } }
/* FP:mini_core.rs-0273 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0137
/* FP:mini_core.rs-0274 */ # [lang = "exchange_malloc"] unsafe fn allocate (size : usize , _align : usize) -> * mut u8 { libc :: malloc (size) }
/* FP:mini_core.rs-0275 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0138
/* FP:mini_core.rs-0276 */ # [lang = "drop"] pub trait Drop { fn drop (& mut self) ; }
/* FP:mini_core.rs-0277 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0139
/* FP:mini_core.rs-0278 */ # [lang = "manually_drop"] # [repr (transparent)] pub struct ManuallyDrop < T : ? Sized > { pub value : T , }
/* FP:mini_core.rs-0279 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_OTHER_0140
/* FP:mini_core.rs-0280 */ # [lang = "maybe_uninit"] # [repr (transparent)] pub union MaybeUninit < T > { pub uninit : () , pub value : ManuallyDrop < T > , }
/* FP:mini_core.rs-0281 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_MOD_0141
/* FP:mini_core.rs-0283 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_MOD_0142
/* FP:mini_core.rs-0285 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_TRAIT_0143
/* FP:mini_core.rs-0286 */ # [lang = "index"] pub trait Index < Idx : ? Sized > { type Output : ? Sized ; fn index (& self , index : Idx) -> & Self :: Output ; }
/* FP:mini_core.rs-0287 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0144
/* FP:mini_core.rs-0288 */ impl < T > Index < usize > for [T ; 3] { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self [index] } }
/* FP:mini_core.rs-0289 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_IMPL_0145
/* FP:mini_core.rs-0290 */ impl < T > Index < usize > for [T] { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self [index] } }
/* FP:mini_core.rs-0291 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_OTHER_0146
/* FP:mini_core.rs-0292 */ extern "C" { type VaListImpl ; }
/* FP:mini_core.rs-0293 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0147
/* FP:mini_core.rs-0294 */ # [lang = "va_list"] # [repr (transparent)] pub struct VaList < 'a > (& 'a mut VaListImpl) ;
/* FP:mini_core.rs-0295 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_OTHER_0148
/* FP:mini_core.rs-0296 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro stringify ($ ($ t : tt) *) { }
/* FP:mini_core.rs-0297 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_OTHER_0149
/* FP:mini_core.rs-0298 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro file () { }
/* FP:mini_core.rs-0299 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_OTHER_0150
/* FP:mini_core.rs-0300 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro line () { }
/* FP:mini_core.rs-0301 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_OTHER_0151
/* FP:mini_core.rs-0302 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro cfg () { }
/* FP:mini_core.rs-0303 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STATIC_0152
/* FP:mini_core.rs-0304 */ pub static A_STATIC : u8 = 42 ;
/* FP:mini_core.rs-0305 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_STRUCT_0153
/* FP:mini_core.rs-0306 */ # [lang = "panic_location"] struct PanicLocation { file : & 'static str , line : u32 , column : u32 , }
/* FP:mini_core.rs-0307 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_gcc_example_mini_core_FN_0154
/* FP:mini_core.rs-0308 */ # [unsafe (no_mangle)] pub fn get_tls () -> u8 { # [thread_local] static A : u8 = 42 ; A }