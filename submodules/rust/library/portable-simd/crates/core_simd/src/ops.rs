mkuse!{use crate :: simd :: { LaneCount , Simd , SimdElement , SupportedLaneCount , cmp :: SimdPartialEq } ;}
mkuse!{use core :: ops :: { Add , Mul } ;}
mkuse!{use core :: ops :: { BitAnd , BitOr , BitXor } ;}
mkuse!{use core :: ops :: { Div , Rem , Sub } ;}
mkuse!{use core :: ops :: { Shl , Shr } ;}
mkmod!{assign, { 
                getname!(assign);
                getsrc!(assign);
                getpath!(assign);
                get_deps!(assign);
                get_crates!(assign);
                mkinclude!(assign);
                 
            }}
mkmod!{deref, { 
                getname!(deref);
                getsrc!(deref);
                getpath!(deref);
                get_deps!(deref);
                get_crates!(deref);
                mkinclude!(deref);
                 
            }}
mkmod!{shift_scalar, { 
                getname!(shift_scalar);
                getsrc!(shift_scalar);
                getpath!(shift_scalar);
                get_deps!(shift_scalar);
                get_crates!(shift_scalar);
                mkinclude!(shift_scalar);
                 
            }}
mkmod!{unary, { 
                getname!(unary);
                getsrc!(unary);
                getpath!(unary);
                get_deps!(unary);
                get_crates!(unary);
                mkinclude!(unary);
                 
            }}
mkitem!{mkimpl!{impl < I , T , const N : usize > core :: ops :: Index < I > for Simd < T , N > where T : SimdElement , LaneCount < N > : SupportedLaneCount , I : core :: slice :: SliceIndex < [T] > , { type Output = I :: Output ; # [inline] fn index (& self , index : I) -> & Self :: Output { & self . as_array () [index] } }}}
mkitem!{mkimpl!{impl < I , T , const N : usize > core :: ops :: IndexMut < I > for Simd < T , N > where T : SimdElement , LaneCount < N > : SupportedLaneCount , I : core :: slice :: SliceIndex < [T] > , { # [inline] fn index_mut (& mut self , index : I) -> & mut Self :: Output { & mut self . as_mut_array () [index] } }}}
mkitem!{macro_rules ! unsafe_base { ($ lhs : ident , $ rhs : ident , { $ simd_call : ident } , $ ($ _ : tt) *) => { unsafe { core :: intrinsics :: simd ::$ simd_call ($ lhs , $ rhs) } } ; }}
mkitem!{# [doc = " SAFETY: This macro should not be used for anything except Shl or Shr, and passed the appropriate shift intrinsic."] # [doc = " It handles performing a bitand in addition to calling the shift operator, so that the result"] # [doc = " is well-defined: LLVM can return a poison value if you shl, lshr, or ashr if `rhs >= <Int>::BITS`"] # [doc = " At worst, this will maybe add another instruction and cycle,"] # [doc = " at best, it may open up more optimization opportunities,"] # [doc = " or simply be elided entirely, especially for SIMD ISAs which default to this."] # [doc = ""] macro_rules ! wrap_bitshift { ($ lhs : ident , $ rhs : ident , { $ simd_call : ident } , $ int : ident) => { # [allow (clippy :: suspicious_arithmetic_impl)] unsafe { core :: intrinsics :: simd ::$ simd_call ($ lhs , $ rhs . bitand (Simd :: splat (<$ int >:: BITS as $ int - 1)) ,) } } ; }}
mkitem!{# [doc = " SAFETY: This macro must only be used to impl Div or Rem and given the matching intrinsic."] # [doc = " It guards against LLVM's UB conditions for integer div or rem using masks and selects,"] # [doc = " thus guaranteeing a Rust value returns instead."] # [doc = ""] # [doc = " |                  | LLVM | Rust"] # [doc = " | :--------------: | :--- | :----------"] # [doc = " | N {/,%} 0        | UB   | panic!()"] # [doc = " | <$int>::MIN / -1 | UB   | <$int>::MIN"] # [doc = " | <$int>::MIN % -1 | UB   | 0"] # [doc = ""] macro_rules ! int_divrem_guard { ($ lhs : ident , $ rhs : ident , { const PANIC_ZERO : &'static str = $ zero : literal ; $ simd_call : ident , $ op : tt } , $ int : ident) => { if $ rhs . simd_eq (Simd :: splat (0 as _)) . any () { panic ! ($ zero) ; } else { let rhs = if <$ int >:: MIN != 0 { ($ lhs . simd_eq (Simd :: splat (<$ int >:: MIN)) & $ rhs . simd_eq (Simd :: splat (- 1i64 as _))) . select (Simd :: splat (1 as _) , $ rhs) } else { $ rhs } ; # [cfg (target_arch = "aarch64")] { let mut out = Simd :: splat (0 as _) ; for i in 0 .. Self :: LEN { out [i] = $ lhs [i] $ op rhs [i] ; } out } # [cfg (not (target_arch = "aarch64"))] { unsafe { core :: intrinsics :: simd ::$ simd_call ($ lhs , rhs) } } } } ; }}
mkitem!{macro_rules ! for_base_types { (T = ($ ($ scalar : ident) ,*) ; type Lhs = Simd < T , N >; type Rhs = Simd < T , N >; type Output = $ out : ty ; impl $ op : ident ::$ call : ident { $ macro_impl : ident $ inner : tt }) => { $ (impl < const N : usize > $ op < Self > for Simd <$ scalar , N > where $ scalar : SimdElement , LaneCount < N >: SupportedLaneCount , { type Output = $ out ; # [inline] # [track_caller] fn $ call (self , rhs : Self) -> Self :: Output { $ macro_impl ! (self , rhs , $ inner , $ scalar) } }) * } }}
mkitem!{macro_rules ! for_base_ops { (T = $ types : tt ; type Lhs = Simd < T , N >; type Rhs = Simd < T , N >; type Output = $ out : ident ; impl $ op : ident ::$ call : ident $ inner : tt $ ($ rest : tt) *) => { for_base_types ! { T = $ types ; type Lhs = Simd < T , N >; type Rhs = Simd < T , N >; type Output = $ out ; impl $ op ::$ call $ inner } for_base_ops ! { T = $ types ; type Lhs = Simd < T , N >; type Rhs = Simd < T , N >; type Output = $ out ; $ ($ rest) * } } ; ($ ($ done : tt) *) => { } }}
mkitem!{for_base_ops ! { T = (i8 , i16 , i32 , i64 , isize , u8 , u16 , u32 , u64 , usize) ; type Lhs = Simd < T , N >; type Rhs = Simd < T , N >; type Output = Self ; impl Add :: add { unsafe_base { simd_add } } impl Mul :: mul { unsafe_base { simd_mul } } impl Sub :: sub { unsafe_base { simd_sub } } impl BitAnd :: bitand { unsafe_base { simd_and } } impl BitOr :: bitor { unsafe_base { simd_or } } impl BitXor :: bitxor { unsafe_base { simd_xor } } impl Div :: div { int_divrem_guard { const PANIC_ZERO : &'static str = "attempt to divide by zero" ; simd_div , / } } impl Rem :: rem { int_divrem_guard { const PANIC_ZERO : &'static str = "attempt to calculate the remainder with a divisor of zero" ; simd_rem , % } } impl Shl :: shl { wrap_bitshift { simd_shl } } impl Shr :: shr { wrap_bitshift { simd_shr } } }}
mkitem!{for_base_ops ! { T = (f32 , f64) ; type Lhs = Simd < T , N >; type Rhs = Simd < T , N >; type Output = Self ; impl Add :: add { unsafe_base { simd_add } } impl Mul :: mul { unsafe_base { simd_mul } } impl Sub :: sub { unsafe_base { simd_sub } } impl Div :: div { unsafe_base { simd_div } } impl Rem :: rem { unsafe_base { simd_rem } } }}