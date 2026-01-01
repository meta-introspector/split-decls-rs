/* FP:mini_core.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0001
/* FP:mini_core.rs-0002 */ # [feature (no_core , lang_items , intrinsics , unboxed_closures , extern_types , decl_macro , rustc_attrs , transparent_unions , auto_traits , freeze_impls , thread_local)] # [no_core] # [allow (dead_code , internal_features , ambiguous_wide_pointer_comparisons)] # [lang = "pointee_sized"] pub trait PointeeSized { }
/* FP:mini_core.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0002
/* FP:mini_core.rs-0004 */ # [lang = "meta_sized"] pub trait MetaSized : PointeeSized { }
/* FP:mini_core.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0003
/* FP:mini_core.rs-0006 */ # [lang = "sized"] pub trait Sized : MetaSized { }
/* FP:mini_core.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0004
/* FP:mini_core.rs-0008 */ # [lang = "destruct"] pub trait Destruct { }
/* FP:mini_core.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0005
/* FP:mini_core.rs-0010 */ # [lang = "tuple_trait"] pub trait Tuple { }
/* FP:mini_core.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0006
/* FP:mini_core.rs-0012 */ # [lang = "unsize"] pub trait Unsize < T : PointeeSized > : PointeeSized { }
/* FP:mini_core.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0007
/* FP:mini_core.rs-0014 */ # [lang = "coerce_unsized"] pub trait CoerceUnsized < T > { }
/* FP:mini_core.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0008
/* FP:mini_core.rs-0016 */ impl < 'a , 'b : 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < & 'a U > for & 'b T { }
/* FP:mini_core.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0009
/* FP:mini_core.rs-0018 */ impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < & 'a mut U > for & 'a mut T { }
/* FP:mini_core.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0010
/* FP:mini_core.rs-0020 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < * const U > for * const T { }
/* FP:mini_core.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0011
/* FP:mini_core.rs-0022 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > CoerceUnsized < * mut U > for * mut T { }
/* FP:mini_core.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0012
/* FP:mini_core.rs-0024 */ # [lang = "dispatch_from_dyn"] pub trait DispatchFromDyn < T > { }
/* FP:mini_core.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0013
/* FP:mini_core.rs-0026 */ impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < & 'a U > for & 'a T { }
/* FP:mini_core.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0014
/* FP:mini_core.rs-0028 */ impl < 'a , T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < & 'a mut U > for & 'a mut T { }
/* FP:mini_core.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0015
/* FP:mini_core.rs-0030 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < * const U > for * const T { }
/* FP:mini_core.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0016
/* FP:mini_core.rs-0032 */ impl < T : PointeeSized + Unsize < U > , U : PointeeSized > DispatchFromDyn < * mut U > for * mut T { }
/* FP:mini_core.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0017
/* FP:mini_core.rs-0034 */ impl < T : MetaSized + Unsize < U > , U : MetaSized > DispatchFromDyn < Box < U > > for Box < T > { }
/* FP:mini_core.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0018
/* FP:mini_core.rs-0036 */ # [lang = "legacy_receiver"] pub trait LegacyReceiver { }
/* FP:mini_core.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0019
/* FP:mini_core.rs-0038 */ impl < T : PointeeSized > LegacyReceiver for & T { }
/* FP:mini_core.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0020
/* FP:mini_core.rs-0040 */ impl < T : PointeeSized > LegacyReceiver for & mut T { }
/* FP:mini_core.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0021
/* FP:mini_core.rs-0042 */ impl < T : MetaSized > LegacyReceiver for Box < T > { }
/* FP:mini_core.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0022
/* FP:mini_core.rs-0044 */ # [lang = "copy"] pub trait Copy { }
/* FP:mini_core.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0023
/* FP:mini_core.rs-0046 */ # [lang = "bikeshed_guaranteed_no_drop"] pub trait BikeshedGuaranteedNoDrop { }
/* FP:mini_core.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0024
/* FP:mini_core.rs-0048 */ impl Copy for bool { }
/* FP:mini_core.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0025
/* FP:mini_core.rs-0050 */ impl Copy for u8 { }
/* FP:mini_core.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0026
/* FP:mini_core.rs-0052 */ impl Copy for u16 { }
/* FP:mini_core.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0027
/* FP:mini_core.rs-0054 */ impl Copy for u32 { }
/* FP:mini_core.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0028
/* FP:mini_core.rs-0056 */ impl Copy for u64 { }
/* FP:mini_core.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0029
/* FP:mini_core.rs-0058 */ impl Copy for u128 { }
/* FP:mini_core.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0030
/* FP:mini_core.rs-0060 */ impl Copy for usize { }
/* FP:mini_core.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0031
/* FP:mini_core.rs-0062 */ impl Copy for i8 { }
/* FP:mini_core.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0032
/* FP:mini_core.rs-0064 */ impl Copy for i16 { }
/* FP:mini_core.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0033
/* FP:mini_core.rs-0066 */ impl Copy for i32 { }
/* FP:mini_core.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0034
/* FP:mini_core.rs-0068 */ impl Copy for isize { }
/* FP:mini_core.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0035
/* FP:mini_core.rs-0070 */ impl Copy for f32 { }
/* FP:mini_core.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0036
/* FP:mini_core.rs-0072 */ impl Copy for f64 { }
/* FP:mini_core.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0037
/* FP:mini_core.rs-0074 */ impl Copy for char { }
/* FP:mini_core.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0038
/* FP:mini_core.rs-0076 */ impl < 'a , T : PointeeSized > Copy for & 'a T { }
/* FP:mini_core.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0039
/* FP:mini_core.rs-0078 */ impl < T : PointeeSized > Copy for * const T { }
/* FP:mini_core.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0040
/* FP:mini_core.rs-0080 */ impl < T : PointeeSized > Copy for * mut T { }
/* FP:mini_core.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0041
/* FP:mini_core.rs-0082 */ impl < T : Copy > Copy for Option < T > { }
/* FP:mini_core.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0042
/* FP:mini_core.rs-0084 */ # [lang = "sync"] pub unsafe trait Sync { }
/* FP:mini_core.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0043
/* FP:mini_core.rs-0086 */ unsafe impl Sync for bool { }
/* FP:mini_core.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0044
/* FP:mini_core.rs-0088 */ unsafe impl Sync for u8 { }
/* FP:mini_core.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0045
/* FP:mini_core.rs-0090 */ unsafe impl Sync for u16 { }
/* FP:mini_core.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0046
/* FP:mini_core.rs-0092 */ unsafe impl Sync for u32 { }
/* FP:mini_core.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0047
/* FP:mini_core.rs-0094 */ unsafe impl Sync for u64 { }
/* FP:mini_core.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0048
/* FP:mini_core.rs-0096 */ unsafe impl Sync for usize { }
/* FP:mini_core.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0049
/* FP:mini_core.rs-0098 */ unsafe impl Sync for i8 { }
/* FP:mini_core.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0050
/* FP:mini_core.rs-0100 */ unsafe impl Sync for i16 { }
/* FP:mini_core.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0051
/* FP:mini_core.rs-0102 */ unsafe impl Sync for i32 { }
/* FP:mini_core.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0052
/* FP:mini_core.rs-0104 */ unsafe impl Sync for isize { }
/* FP:mini_core.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0053
/* FP:mini_core.rs-0106 */ unsafe impl Sync for char { }
/* FP:mini_core.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0054
/* FP:mini_core.rs-0108 */ unsafe impl Sync for f32 { }
/* FP:mini_core.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0055
/* FP:mini_core.rs-0110 */ unsafe impl < 'a , T : PointeeSized > Sync for & 'a T { }
/* FP:mini_core.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0056
/* FP:mini_core.rs-0112 */ unsafe impl < T : Sync , const N : usize > Sync for [T ; N] { }
/* FP:mini_core.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0057
/* FP:mini_core.rs-0114 */ # [lang = "freeze"] unsafe auto trait Freeze { }
/* FP:mini_core.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0058
/* FP:mini_core.rs-0116 */ unsafe impl < T : PointeeSized > Freeze for PhantomData < T > { }
/* FP:mini_core.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0059
/* FP:mini_core.rs-0118 */ unsafe impl < T : PointeeSized > Freeze for * const T { }
/* FP:mini_core.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0060
/* FP:mini_core.rs-0120 */ unsafe impl < T : PointeeSized > Freeze for * mut T { }
/* FP:mini_core.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0061
/* FP:mini_core.rs-0122 */ unsafe impl < T : PointeeSized > Freeze for & T { }
/* FP:mini_core.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0062
/* FP:mini_core.rs-0124 */ unsafe impl < T : PointeeSized > Freeze for & mut T { }
/* FP:mini_core.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0063
/* FP:mini_core.rs-0126 */ # [lang = "structural_peq"] pub trait StructuralPartialEq { }
/* FP:mini_core.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0064
/* FP:mini_core.rs-0128 */ # [lang = "not"] pub trait Not { type Output ; fn not (self) -> Self :: Output ; }
/* FP:mini_core.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0065
/* FP:mini_core.rs-0130 */ impl Not for bool { type Output = bool ; fn not (self) -> bool { ! self } }
/* FP:mini_core.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0066
/* FP:mini_core.rs-0132 */ # [lang = "mul"] pub trait Mul < RHS = Self > { type Output ; # [must_use] fn mul (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0067
/* FP:mini_core.rs-0134 */ impl Mul for u8 { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }
/* FP:mini_core.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0068
/* FP:mini_core.rs-0136 */ impl Mul for usize { type Output = Self ; fn mul (self , rhs : Self) -> Self :: Output { self * rhs } }
/* FP:mini_core.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0069
/* FP:mini_core.rs-0138 */ # [lang = "add"] pub trait Add < RHS = Self > { type Output ; fn add (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0070
/* FP:mini_core.rs-0140 */ impl Add for u8 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0071
/* FP:mini_core.rs-0142 */ impl Add for i8 { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0072
/* FP:mini_core.rs-0144 */ impl Add for usize { type Output = Self ; fn add (self , rhs : Self) -> Self { self + rhs } }
/* FP:mini_core.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0073
/* FP:mini_core.rs-0146 */ # [lang = "sub"] pub trait Sub < RHS = Self > { type Output ; fn sub (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0074
/* FP:mini_core.rs-0148 */ impl Sub for usize { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0075
/* FP:mini_core.rs-0150 */ impl Sub for u8 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0076
/* FP:mini_core.rs-0152 */ impl Sub for i8 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0077
/* FP:mini_core.rs-0154 */ impl Sub for i16 { type Output = Self ; fn sub (self , rhs : Self) -> Self { self - rhs } }
/* FP:mini_core.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0078
/* FP:mini_core.rs-0156 */ # [lang = "rem"] pub trait Rem < RHS = Self > { type Output ; fn rem (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0079
/* FP:mini_core.rs-0158 */ impl Rem for usize { type Output = Self ; fn rem (self , rhs : Self) -> Self { self % rhs } }
/* FP:mini_core.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0080
/* FP:mini_core.rs-0160 */ # [lang = "bitor"] pub trait BitOr < RHS = Self > { type Output ; # [must_use] fn bitor (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0081
/* FP:mini_core.rs-0162 */ impl BitOr for bool { type Output = bool ; fn bitor (self , rhs : bool) -> bool { self | rhs } }
/* FP:mini_core.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0082
/* FP:mini_core.rs-0164 */ impl < 'a > BitOr < bool > for & 'a bool { type Output = bool ; fn bitor (self , rhs : bool) -> bool { * self | rhs } }
/* FP:mini_core.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0083
/* FP:mini_core.rs-0166 */ # [lang = "eq"] pub trait PartialEq < Rhs : ? Sized = Self > { fn eq (& self , other : & Rhs) -> bool ; fn ne (& self , other : & Rhs) -> bool ; }
/* FP:mini_core.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0084
/* FP:mini_core.rs-0168 */ impl PartialEq for u8 { fn eq (& self , other : & u8) -> bool { (* self) == (* other) } fn ne (& self , other : & u8) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0085
/* FP:mini_core.rs-0170 */ impl PartialEq for u16 { fn eq (& self , other : & u16) -> bool { (* self) == (* other) } fn ne (& self , other : & u16) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0086
/* FP:mini_core.rs-0172 */ impl PartialEq for u32 { fn eq (& self , other : & u32) -> bool { (* self) == (* other) } fn ne (& self , other : & u32) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0087
/* FP:mini_core.rs-0174 */ impl PartialEq for u64 { fn eq (& self , other : & u64) -> bool { (* self) == (* other) } fn ne (& self , other : & u64) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0088
/* FP:mini_core.rs-0176 */ impl PartialEq for u128 { fn eq (& self , other : & u128) -> bool { (* self) == (* other) } fn ne (& self , other : & u128) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0177 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0089
/* FP:mini_core.rs-0178 */ impl PartialEq for usize { fn eq (& self , other : & usize) -> bool { (* self) == (* other) } fn ne (& self , other : & usize) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0179 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0090
/* FP:mini_core.rs-0180 */ impl PartialEq for i8 { fn eq (& self , other : & i8) -> bool { (* self) == (* other) } fn ne (& self , other : & i8) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0181 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0091
/* FP:mini_core.rs-0182 */ impl PartialEq for i32 { fn eq (& self , other : & i32) -> bool { (* self) == (* other) } fn ne (& self , other : & i32) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0183 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0092
/* FP:mini_core.rs-0184 */ impl PartialEq for isize { fn eq (& self , other : & isize) -> bool { (* self) == (* other) } fn ne (& self , other : & isize) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0185 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0093
/* FP:mini_core.rs-0186 */ impl PartialEq for char { fn eq (& self , other : & char) -> bool { (* self) == (* other) } fn ne (& self , other : & char) -> bool { (* self) != (* other) } }
/* FP:mini_core.rs-0187 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0094
/* FP:mini_core.rs-0188 */ impl < T : ? Sized > PartialEq for * const T { fn eq (& self , other : & * const T) -> bool { * self == * other } fn ne (& self , other : & * const T) -> bool { * self != * other } }
/* FP:mini_core.rs-0189 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0095
/* FP:mini_core.rs-0190 */ impl < T : PartialEq > PartialEq for Option < T > { fn eq (& self , other : & Self) -> bool { match (self , other) { (Some (lhs) , Some (rhs)) => * lhs == * rhs , (None , None) => true , _ => false , } } fn ne (& self , other : & Self) -> bool { match (self , other) { (Some (lhs) , Some (rhs)) => * lhs != * rhs , (None , None) => false , _ => true , } } }
/* FP:mini_core.rs-0191 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0096
/* FP:mini_core.rs-0192 */ # [lang = "shl"] pub trait Shl < RHS = Self > { type Output ; # [must_use] fn shl (self , rhs : RHS) -> Self :: Output ; }
/* FP:mini_core.rs-0193 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0097
/* FP:mini_core.rs-0194 */ impl Shl for u128 { type Output = u128 ; fn shl (self , rhs : u128) -> u128 { self << rhs } }
/* FP:mini_core.rs-0195 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0098
/* FP:mini_core.rs-0196 */ # [lang = "neg"] pub trait Neg { type Output ; fn neg (self) -> Self :: Output ; }
/* FP:mini_core.rs-0197 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0099
/* FP:mini_core.rs-0198 */ impl Neg for i8 { type Output = i8 ; fn neg (self) -> i8 { - self } }
/* FP:mini_core.rs-0199 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0100
/* FP:mini_core.rs-0200 */ impl Neg for i16 { type Output = i16 ; fn neg (self) -> i16 { self } }
/* FP:mini_core.rs-0201 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0101
/* FP:mini_core.rs-0202 */ impl Neg for isize { type Output = isize ; fn neg (self) -> isize { - self } }
/* FP:mini_core.rs-0203 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0102
/* FP:mini_core.rs-0204 */ impl Neg for f32 { type Output = f32 ; fn neg (self) -> f32 { - self } }
/* FP:mini_core.rs-0205 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_ENUM_0103
/* FP:mini_core.rs-0206 */ pub enum Option < T > { Some (T) , None , }
/* FP:mini_core.rs-0207 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_USE_0104
/* FP:mini_core.rs-0208 */ pub use Option :: * ;
/* FP:mini_core.rs-0209 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0105
/* FP:mini_core.rs-0210 */ # [lang = "phantom_data"] pub struct PhantomData < T : PointeeSized > ;
/* FP:mini_core.rs-0211 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0106
/* FP:mini_core.rs-0212 */ # [lang = "fn_once"] # [rustc_paren_sugar] pub trait FnOnce < Args : Tuple > { # [lang = "fn_once_output"] type Output ; extern "rust-call" fn call_once (self , args : Args) -> Self :: Output ; }
/* FP:mini_core.rs-0213 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0107
/* FP:mini_core.rs-0214 */ # [lang = "fn_mut"] # [rustc_paren_sugar] pub trait FnMut < Args : Tuple > : FnOnce < Args > { extern "rust-call" fn call_mut (& mut self , args : Args) -> Self :: Output ; }
/* FP:mini_core.rs-0215 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0108
/* FP:mini_core.rs-0216 */ # [lang = "panic"] # [track_caller] pub fn panic (_msg : & 'static str) -> ! { unsafe { libc :: puts ("Panicking\n\0" as * const str as * const i8) ; intrinsics :: abort () ; } }
/* FP:mini_core.rs-0217 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_MACRO_0109
/* FP:mini_core.rs-0219 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_MACRO_0110
/* FP:mini_core.rs-0220 */ panic_const ! { panic_const_add_overflow = "attempt to add with overflow" , panic_const_sub_overflow = "attempt to subtract with overflow" , panic_const_mul_overflow = "attempt to multiply with overflow" , panic_const_div_overflow = "attempt to divide with overflow" , panic_const_rem_overflow = "attempt to calculate the remainder with overflow" , panic_const_neg_overflow = "attempt to negate with overflow" , panic_const_shr_overflow = "attempt to shift right with overflow" , panic_const_shl_overflow = "attempt to shift left with overflow" , panic_const_div_by_zero = "attempt to divide by zero" , panic_const_rem_by_zero = "attempt to calculate the remainder with a divisor of zero" , }
/* FP:mini_core.rs-0221 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0111
/* FP:mini_core.rs-0222 */ # [lang = "panic_bounds_check"] # [track_caller] fn panic_bounds_check (index : usize , len : usize) -> ! { unsafe { libc :: printf ("index out of bounds: the len is %d but the index is %d\n\0" as * const str as * const i8 , len , index ,) ; intrinsics :: abort () ; } }
/* FP:mini_core.rs-0223 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0112
/* FP:mini_core.rs-0224 */ # [lang = "panic_cannot_unwind"] # [track_caller] fn panic_cannot_unwind () -> ! { unsafe { libc :: puts ("panic in a function that cannot unwind\n\0" as * const str as * const i8) ; intrinsics :: abort () ; } }
/* FP:mini_core.rs-0225 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0113
/* FP:mini_core.rs-0226 */ # [lang = "eh_personality"] fn eh_personality (_version : i32 , _actions : i32 , _exception_class : u64 , _exception_object : * mut () , _context : * mut () ,) -> i32 { loop { } }
/* FP:mini_core.rs-0227 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0114
/* FP:mini_core.rs-0228 */ # [lang = "panic_in_cleanup"] fn panic_in_cleanup () -> ! { loop { } }
/* FP:mini_core.rs-0229 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0115
/* FP:mini_core.rs-0230 */ # [cfg (all (unix , not (target_vendor = "apple")))] # [link (name = "gcc_s")] unsafe extern "C" { fn _Unwind_Resume (exc : * mut ()) -> ! ; }
/* FP:mini_core.rs-0231 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0116
/* FP:mini_core.rs-0232 */ # [lang = "drop_in_place"] # [allow (unconditional_recursion)] pub unsafe fn drop_in_place < T : ? Sized > (to_drop : * mut T) { drop_in_place (to_drop) ; }
/* FP:mini_core.rs-0233 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0117
/* FP:mini_core.rs-0234 */ # [lang = "unpin"] pub auto trait Unpin { }
/* FP:mini_core.rs-0235 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0118
/* FP:mini_core.rs-0236 */ # [lang = "deref"] pub trait Deref { type Target : ? Sized ; fn deref (& self) -> & Self :: Target ; }
/* FP:mini_core.rs-0237 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0119
/* FP:mini_core.rs-0238 */ # [repr (transparent)] # [rustc_layout_scalar_valid_range_start (1)] # [rustc_nonnull_optimization_guaranteed] pub struct NonNull < T : PointeeSized > (pub * const T) ;
/* FP:mini_core.rs-0239 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0120
/* FP:mini_core.rs-0240 */ impl < T : PointeeSized , U : PointeeSized > CoerceUnsized < NonNull < U > > for NonNull < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0241 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0121
/* FP:mini_core.rs-0242 */ impl < T : PointeeSized , U : PointeeSized > DispatchFromDyn < NonNull < U > > for NonNull < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0243 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0122
/* FP:mini_core.rs-0244 */ pub struct Unique < T : PointeeSized > { pub pointer : NonNull < T > , pub _marker : PhantomData < T > , }
/* FP:mini_core.rs-0245 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0123
/* FP:mini_core.rs-0246 */ impl < T : PointeeSized , U : PointeeSized > CoerceUnsized < Unique < U > > for Unique < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0247 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0124
/* FP:mini_core.rs-0248 */ impl < T : PointeeSized , U : PointeeSized > DispatchFromDyn < Unique < U > > for Unique < T > where T : Unsize < U > { }
/* FP:mini_core.rs-0249 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0125
/* FP:mini_core.rs-0250 */ # [lang = "global_alloc_ty"] pub struct Global ;
/* FP:mini_core.rs-0251 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0126
/* FP:mini_core.rs-0252 */ # [lang = "owned_box"] pub struct Box < T : ? Sized , A = Global > (Unique < T > , A) ;
/* FP:mini_core.rs-0253 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0127
/* FP:mini_core.rs-0254 */ impl < T : ? Sized + Unsize < U > , U : ? Sized > CoerceUnsized < Box < U > > for Box < T > { }
/* FP:mini_core.rs-0255 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0128
/* FP:mini_core.rs-0256 */ impl < T > Box < T > { pub fn new (val : T) -> Box < T > { unsafe { let size = intrinsics :: size_of :: < T > () ; let ptr = libc :: malloc (size) ; intrinsics :: copy (& val as * const T as * const u8 , ptr , size) ; Box (Unique { pointer : NonNull (ptr as * const T) , _marker : PhantomData } , Global) } } }
/* FP:mini_core.rs-0257 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0129
/* FP:mini_core.rs-0258 */ impl < T : ? Sized , A > Drop for Box < T , A > { fn drop (& mut self) { unsafe { libc :: free (self . 0 . pointer . 0 as * mut u8) ; } } }
/* FP:mini_core.rs-0259 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0130
/* FP:mini_core.rs-0260 */ impl < T : ? Sized > Deref for Box < T > { type Target = T ; fn deref (& self) -> & Self :: Target { & * * self } }
/* FP:mini_core.rs-0261 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0131
/* FP:mini_core.rs-0262 */ # [lang = "exchange_malloc"] unsafe fn allocate (size : usize , _align : usize) -> * mut u8 { libc :: malloc (size) }
/* FP:mini_core.rs-0263 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0132
/* FP:mini_core.rs-0264 */ # [lang = "drop"] pub trait Drop { fn drop (& mut self) ; }
/* FP:mini_core.rs-0265 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0133
/* FP:mini_core.rs-0266 */ # [lang = "manually_drop"] # [repr (transparent)] pub struct ManuallyDrop < T : ? Sized > { pub value : T , }
/* FP:mini_core.rs-0267 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0134
/* FP:mini_core.rs-0268 */ # [lang = "maybe_uninit"] # [repr (transparent)] pub union MaybeUninit < T > { pub uninit : () , pub value : ManuallyDrop < T > , }
/* FP:mini_core.rs-0269 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_MOD_0135
/* FP:mini_core.rs-0271 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_MOD_0136
/* FP:mini_core.rs-0273 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_TRAIT_0137
/* FP:mini_core.rs-0274 */ # [lang = "index"] pub trait Index < Idx : ? Sized > { type Output : ? Sized ; fn index (& self , index : Idx) -> & Self :: Output ; }
/* FP:mini_core.rs-0275 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0138
/* FP:mini_core.rs-0276 */ impl < T > Index < usize > for [T ; 3] { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self [index] } }
/* FP:mini_core.rs-0277 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_IMPL_0139
/* FP:mini_core.rs-0278 */ impl < T > Index < usize > for [T] { type Output = T ; fn index (& self , index : usize) -> & Self :: Output { & self [index] } }
/* FP:mini_core.rs-0279 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0140
/* FP:mini_core.rs-0280 */ unsafe extern "C" { type VaListImpl ; }
/* FP:mini_core.rs-0281 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0141
/* FP:mini_core.rs-0282 */ # [lang = "va_list"] # [repr (transparent)] pub struct VaList < 'a > (& 'a mut VaListImpl) ;
/* FP:mini_core.rs-0283 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0142
/* FP:mini_core.rs-0284 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro stringify ($ ($ t : tt) *) { }
/* FP:mini_core.rs-0285 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0143
/* FP:mini_core.rs-0286 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro file () { }
/* FP:mini_core.rs-0287 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0144
/* FP:mini_core.rs-0288 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro line () { }
/* FP:mini_core.rs-0289 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0145
/* FP:mini_core.rs-0290 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro cfg () { }
/* FP:mini_core.rs-0291 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0146
/* FP:mini_core.rs-0292 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro asm () { }
/* FP:mini_core.rs-0293 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0147
/* FP:mini_core.rs-0294 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro global_asm () { }
/* FP:mini_core.rs-0295 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_OTHER_0148
/* FP:mini_core.rs-0296 */ # [rustc_builtin_macro] # [rustc_macro_transparency = "semitransparent"] pub macro naked_asm () { }
/* FP:mini_core.rs-0297 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STATIC_0149
/* FP:mini_core.rs-0298 */ pub static A_STATIC : u8 = 42 ;
/* FP:mini_core.rs-0299 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_STRUCT_0150
/* FP:mini_core.rs-0300 */ # [lang = "panic_location"] struct PanicLocation { file : & 'static str , line : u32 , column : u32 , }
/* FP:mini_core.rs-0301 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_mini_core_FN_0151
/* FP:mini_core.rs-0302 */ # [unsafe (no_mangle)] # [cfg (not (all (windows , target_env = "gnu")))] pub fn get_tls () -> u8 { # [thread_local] static A : u8 = 42 ; A }