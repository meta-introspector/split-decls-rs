mkuse!{use crate :: { core_arch :: simd :: * , intrinsics :: simd :: * , mem , mem :: transmute } ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkuse!{use super :: macros :: * ;}
mkitem!{types ! { #! [unstable (feature = "stdarch_powerpc" , issue = "111145")] # [doc = " PowerPC-specific 128-bit wide vector of sixteen packed `i8`"] pub struct vector_signed_char (16 x i8) ; # [doc = " PowerPC-specific 128-bit wide vector of sixteen packed `u8`"] pub struct vector_unsigned_char (16 x u8) ; # [doc = " PowerPC-specific 128-bit wide vector mask of sixteen packed elements"] pub struct vector_bool_char (16 x i8) ; # [doc = " PowerPC-specific 128-bit wide vector of eight packed `i16`"] pub struct vector_signed_short (8 x i16) ; # [doc = " PowerPC-specific 128-bit wide vector of eight packed `u16`"] pub struct vector_unsigned_short (8 x u16) ; # [doc = " PowerPC-specific 128-bit wide vector mask of eight packed elements"] pub struct vector_bool_short (8 x i16) ; # [doc = " PowerPC-specific 128-bit wide vector of four packed `i32`"] pub struct vector_signed_int (4 x i32) ; # [doc = " PowerPC-specific 128-bit wide vector of four packed `u32`"] pub struct vector_unsigned_int (4 x u32) ; # [doc = " PowerPC-specific 128-bit wide vector mask of four packed elements"] pub struct vector_bool_int (4 x i32) ; # [doc = " PowerPC-specific 128-bit wide vector of four packed `f32`"] pub struct vector_float (4 x f32) ; }}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.ppc.altivec.lvx"] fn lvx (p : * const i8) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.lvebx"] fn lvebx (p : * const i8) -> vector_signed_char ; # [link_name = "llvm.ppc.altivec.lvehx"] fn lvehx (p : * const i8) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.lvewx"] fn lvewx (p : * const i8) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.lvxl"] fn lvxl (p : * const i8) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.stvx"] fn stvx (a : vector_signed_int , p : * const i8) ; # [link_name = "llvm.ppc.altivec.stvebx"] fn stvebx (a : vector_signed_char , p : * const i8) ; # [link_name = "llvm.ppc.altivec.stvehx"] fn stvehx (a : vector_signed_short , p : * const i8) ; # [link_name = "llvm.ppc.altivec.stvewx"] fn stvewx (a : vector_signed_int , p : * const i8) ; # [link_name = "llvm.ppc.altivec.stvxl"] fn stvxl (a : vector_signed_int , p : * const i8) ; # [link_name = "llvm.ppc.altivec.vperm"] fn vperm (a : vector_signed_int , b : vector_signed_int , c : vector_unsigned_char ,) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vmhaddshs"] fn vmhaddshs (a : vector_signed_short , b : vector_signed_short , c : vector_signed_short ,) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vmhraddshs"] fn vmhraddshs (a : vector_signed_short , b : vector_signed_short , c : vector_signed_short ,) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vmsumuhs"] fn vmsumuhs (a : vector_unsigned_short , b : vector_unsigned_short , c : vector_unsigned_int ,) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vmsumshs"] fn vmsumshs (a : vector_signed_short , b : vector_signed_short , c : vector_signed_int ,) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vmsumubm"] fn vmsumubm (a : vector_unsigned_char , b : vector_unsigned_char , c : vector_unsigned_int ,) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vmsummbm"] fn vmsummbm (a : vector_signed_char , b : vector_unsigned_char , c : vector_signed_int ,) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vmsumuhm"] fn vmsumuhm (a : vector_unsigned_short , b : vector_unsigned_short , c : vector_unsigned_int ,) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vmsumshm"] fn vmsumshm (a : vector_signed_short , b : vector_signed_short , c : vector_signed_int ,) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vnmsubfp"] fn vnmsubfp (a : vector_float , b : vector_float , c : vector_float) -> vector_float ; # [link_name = "llvm.ppc.altivec.vsum2sws"] fn vsum2sws (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vsum4ubs"] fn vsum4ubs (a : vector_unsigned_char , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vsum4sbs"] fn vsum4sbs (a : vector_signed_char , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vsum4shs"] fn vsum4shs (a : vector_signed_short , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vmuleub"] fn vmuleub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vmulesb"] fn vmulesb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vmuleuh"] fn vmuleuh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vmulesh"] fn vmulesh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vmuloub"] fn vmuloub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vmulosb"] fn vmulosb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vmulouh"] fn vmulouh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vmulosh"] fn vmulosh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_int ; # [link_name = "llvm.smax.v16i8"] fn vmaxsb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char ; # [link_name = "llvm.smax.v8i16"] fn vmaxsh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short ; # [link_name = "llvm.smax.v4i32"] fn vmaxsw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.umax.v16i8"] fn vmaxub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char ; # [link_name = "llvm.umax.v8i16"] fn vmaxuh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short ; # [link_name = "llvm.umax.v4i32"] fn vmaxuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.smin.v16i8"] fn vminsb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char ; # [link_name = "llvm.smin.v8i16"] fn vminsh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short ; # [link_name = "llvm.smin.v4i32"] fn vminsw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.umin.v16i8"] fn vminub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char ; # [link_name = "llvm.umin.v8i16"] fn vminuh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short ; # [link_name = "llvm.umin.v4i32"] fn vminuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vsubsbs"] fn vsubsbs (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char ; # [link_name = "llvm.ppc.altivec.vsubshs"] fn vsubshs (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vsubsws"] fn vsubsws (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vsububs"] fn vsububs (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char ; # [link_name = "llvm.ppc.altivec.vsubuhs"] fn vsubuhs (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vsubuws"] fn vsubuws (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vsubcuw"] fn vsubcuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vaddcuw"] fn vaddcuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vaddsbs"] fn vaddsbs (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char ; # [link_name = "llvm.ppc.altivec.vaddshs"] fn vaddshs (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vaddsws"] fn vaddsws (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vaddubs"] fn vaddubs (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char ; # [link_name = "llvm.ppc.altivec.vadduhs"] fn vadduhs (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vadduws"] fn vadduws (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vavgsb"] fn vavgsb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char ; # [link_name = "llvm.ppc.altivec.vavgsh"] fn vavgsh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vavgsw"] fn vavgsw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vavgub"] fn vavgub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char ; # [link_name = "llvm.ppc.altivec.vavguh"] fn vavguh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vavguw"] fn vavguw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vcmpbfp"] fn vcmpbfp (a : vector_float , b : vector_float) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vcmpequb"] fn vcmpequb (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_bool_char ; # [link_name = "llvm.ppc.altivec.vcmpequh"] fn vcmpequh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_bool_short ; # [link_name = "llvm.ppc.altivec.vcmpequw"] fn vcmpequw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_bool_int ; # [link_name = "llvm.ppc.altivec.vcmpneb"] fn vcmpneb (a : vector_signed_char , b : vector_signed_char) -> vector_bool_char ; # [link_name = "llvm.ppc.altivec.vcmpneh"] fn vcmpneh (a : vector_signed_short , b : vector_signed_short) -> vector_bool_short ; # [link_name = "llvm.ppc.altivec.vcmpnew"] fn vcmpnew (a : vector_signed_int , b : vector_signed_int) -> vector_bool_int ; # [link_name = "llvm.ppc.altivec.vcmpgefp"] fn vcmpgefp (a : vector_float , b : vector_float) -> vector_bool_int ; # [link_name = "llvm.ppc.altivec.vcmpgtub"] fn vcmpgtub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_bool_char ; # [link_name = "llvm.ppc.altivec.vcmpgtuh"] fn vcmpgtuh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_bool_short ; # [link_name = "llvm.ppc.altivec.vcmpgtuw"] fn vcmpgtuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_bool_int ; # [link_name = "llvm.ppc.altivec.vcmpgtsb"] fn vcmpgtsb (a : vector_signed_char , b : vector_signed_char) -> vector_bool_char ; # [link_name = "llvm.ppc.altivec.vcmpgtsh"] fn vcmpgtsh (a : vector_signed_short , b : vector_signed_short) -> vector_bool_short ; # [link_name = "llvm.ppc.altivec.vcmpgtsw"] fn vcmpgtsw (a : vector_signed_int , b : vector_signed_int) -> vector_bool_int ; # [link_name = "llvm.ppc.altivec.vexptefp"] fn vexptefp (a : vector_float) -> vector_float ; # [link_name = "llvm.ppc.altivec.vcmpequb.p"] fn vcmpequb_p (cr : i32 , a : vector_unsigned_char , b : vector_unsigned_char) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpequh.p"] fn vcmpequh_p (cr : i32 , a : vector_unsigned_short , b : vector_unsigned_short) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpequw.p"] fn vcmpequw_p (cr : i32 , a : vector_unsigned_int , b : vector_unsigned_int) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpeqfp.p"] fn vcmpeqfp_p (cr : i32 , a : vector_float , b : vector_float) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgtub.p"] fn vcmpgtub_p (cr : i32 , a : vector_unsigned_char , b : vector_unsigned_char) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgtuh.p"] fn vcmpgtuh_p (cr : i32 , a : vector_unsigned_short , b : vector_unsigned_short) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgtuw.p"] fn vcmpgtuw_p (cr : i32 , a : vector_unsigned_int , b : vector_unsigned_int) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgtsb.p"] fn vcmpgtsb_p (cr : i32 , a : vector_signed_char , b : vector_signed_char) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgtsh.p"] fn vcmpgtsh_p (cr : i32 , a : vector_signed_short , b : vector_signed_short) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgtsw.p"] fn vcmpgtsw_p (cr : i32 , a : vector_signed_int , b : vector_signed_int) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgefp.p"] fn vcmpgefp_p (cr : i32 , a : vector_float , b : vector_float) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpgtfp.p"] fn vcmpgtfp_p (cr : i32 , a : vector_float , b : vector_float) -> i32 ; # [link_name = "llvm.ppc.altivec.vcmpbfp.p"] fn vcmpbfp_p (cr : i32 , a : vector_float , b : vector_float) -> i32 ; # [link_name = "llvm.ppc.altivec.vcfsx"] fn vcfsx (a : vector_signed_int , b : i32) -> vector_float ; # [link_name = "llvm.ppc.altivec.vcfux"] fn vcfux (a : vector_unsigned_int , b : i32) -> vector_float ; # [link_name = "llvm.ppc.altivec.vctsxs"] fn vctsxs (a : vector_float , b : i32) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vctuxs"] fn vctuxs (a : vector_float , b : i32) -> vector_unsigned_int ; # [link_name = "llvm.ppc.altivec.vpkshss"] fn vpkshss (a : vector_signed_short , b : vector_signed_short) -> vector_signed_char ; # [link_name = "llvm.ppc.altivec.vpkshus"] fn vpkshus (a : vector_signed_short , b : vector_signed_short) -> vector_unsigned_char ; # [link_name = "llvm.ppc.altivec.vpkuhus"] fn vpkuhus (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_char ; # [link_name = "llvm.ppc.altivec.vpkswss"] fn vpkswss (a : vector_signed_int , b : vector_signed_int) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vpkswus"] fn vpkswus (a : vector_signed_int , b : vector_signed_int) -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vpkuwus"] fn vpkuwus (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vupkhsb"] fn vupkhsb (a : vector_signed_char) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vupklsb"] fn vupklsb (a : vector_signed_char) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vupkhsh"] fn vupkhsh (a : vector_signed_short) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vupklsh"] fn vupklsh (a : vector_signed_short) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.mfvscr"] fn mfvscr () -> vector_unsigned_short ; # [link_name = "llvm.ppc.altivec.vlogefp"] fn vlogefp (a : vector_float) -> vector_float ; # [link_name = "llvm.ppc.altivec.vsl"] fn vsl (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vslo"] fn vslo (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vsrab"] fn vsrab (a : vector_signed_char , b : vector_unsigned_char) -> vector_signed_char ; # [link_name = "llvm.ppc.altivec.vsrah"] fn vsrah (a : vector_signed_short , b : vector_unsigned_short) -> vector_signed_short ; # [link_name = "llvm.ppc.altivec.vsraw"] fn vsraw (a : vector_signed_int , b : vector_unsigned_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vsr"] fn vsr (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vsro"] fn vsro (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int ; # [link_name = "llvm.ppc.altivec.vslv"] fn vslv (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char ; # [link_name = "llvm.ppc.altivec.vsrv"] fn vsrv (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char ; # [link_name = "llvm.nearbyint.v4f32"] fn vrfin (a : vector_float) -> vector_float ; }}
mkitem!{impl_from ! { i8x16 , u8x16 , i16x8 , u16x8 , i32x4 , u32x4 , f32x4 }}
mkitem!{impl_neg ! { i8x16 : 0 }}
mkitem!{impl_neg ! { i16x8 : 0 }}
mkitem!{impl_neg ! { i32x4 : 0 }}
mkitem!{impl_neg ! { f32x4 : 0f32 }}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkuse!{use super :: * ;}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorInsert { type Scalar ; unsafe fn vec_insert < const IDX : u32 > (self , s : Self :: Scalar) -> Self ; }}}

macro_rules! idx_in_vec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function idx_in_vec in module {}", module_path!());
    };
}

mkfn!{
    idx_in_vec_introspect!();
    const fn idx_in_vec < T , const IDX : u32 > () -> u32 { IDX & (16 / crate :: mem :: size_of :: < T > () as u32) }
}
mkitem!{macro_rules ! impl_vec_insert { ($ ty : ident) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorInsert for t_t_l ! ($ ty) { type Scalar = $ ty ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_insert < const IDX : u32 > (self , s : Self :: Scalar) -> Self { simd_insert (self , const { idx_in_vec ::< Self :: Scalar , IDX > () } , s) } } } ; }}
mkitem!{impl_vec_insert ! { i8 }}
mkitem!{impl_vec_insert ! { u8 }}
mkitem!{impl_vec_insert ! { i16 }}
mkitem!{impl_vec_insert ! { u16 }}
mkitem!{impl_vec_insert ! { i32 }}
mkitem!{impl_vec_insert ! { u32 }}
mkitem!{impl_vec_insert ! { f32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorExtract { type Scalar ; unsafe fn vec_extract < const IDX : u32 > (self) -> Self :: Scalar ; }}}
mkitem!{macro_rules ! impl_vec_extract { ($ ty : ident) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorExtract for t_t_l ! ($ ty) { type Scalar = $ ty ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_extract < const IDX : u32 > (self) -> Self :: Scalar { simd_extract (self , const { idx_in_vec ::< Self :: Scalar , IDX > () }) } } } ; }}
mkitem!{impl_vec_extract ! { i8 }}
mkitem!{impl_vec_extract ! { u8 }}
mkitem!{impl_vec_extract ! { i16 }}
mkitem!{impl_vec_extract ! { u16 }}
mkitem!{impl_vec_extract ! { i32 }}
mkitem!{impl_vec_extract ! { u32 }}
mkitem!{impl_vec_extract ! { f32 }}
mkitem!{macro_rules ! impl_vec_cmp { ([$ Trait : ident $ m : ident] ($ b : ident , $ h : ident , $ w : ident)) => { impl_vec_cmp ! { [$ Trait $ m] ($ b , $ b , $ h , $ h , $ w , $ w) } } ; ([$ Trait : ident $ m : ident] ($ ub : ident , $ sb : ident , $ uh : ident , $ sh : ident , $ uw : ident , $ sw : ident)) => { impl_vec_trait ! { [$ Trait $ m] $ ub (vector_unsigned_char , vector_unsigned_char) -> vector_bool_char } impl_vec_trait ! { [$ Trait $ m] $ sb (vector_signed_char , vector_signed_char) -> vector_bool_char } impl_vec_trait ! { [$ Trait $ m] $ uh (vector_unsigned_short , vector_unsigned_short) -> vector_bool_short } impl_vec_trait ! { [$ Trait $ m] $ sh (vector_signed_short , vector_signed_short) -> vector_bool_short } impl_vec_trait ! { [$ Trait $ m] $ uw (vector_unsigned_int , vector_unsigned_int) -> vector_bool_int } impl_vec_trait ! { [$ Trait $ m] $ sw (vector_signed_int , vector_signed_int) -> vector_bool_int } } }}
mkitem!{macro_rules ! impl_vec_any_all { ([$ Trait : ident $ m : ident] ($ b : ident , $ h : ident , $ w : ident)) => { impl_vec_any_all ! { [$ Trait $ m] ($ b , $ b , $ h , $ h , $ w , $ w) } } ; ([$ Trait : ident $ m : ident] ($ ub : ident , $ sb : ident , $ uh : ident , $ sh : ident , $ uw : ident , $ sw : ident)) => { impl_vec_trait ! { [$ Trait $ m] $ ub (vector_unsigned_char , vector_unsigned_char) -> bool } impl_vec_trait ! { [$ Trait $ m] $ sb (vector_signed_char , vector_signed_char) -> bool } impl_vec_trait ! { [$ Trait $ m] $ uh (vector_unsigned_short , vector_unsigned_short) -> bool } impl_vec_trait ! { [$ Trait $ m] $ sh (vector_signed_short , vector_signed_short) -> bool } impl_vec_trait ! { [$ Trait $ m] $ uw (vector_unsigned_int , vector_unsigned_int) -> bool } impl_vec_trait ! { [$ Trait $ m] $ sw (vector_signed_int , vector_signed_int) -> bool } } }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorLd { type Result ; unsafe fn vec_ld (self , off : isize) -> Self :: Result ; unsafe fn vec_ldl (self , off : isize) -> Self :: Result ; }}}
mkitem!{macro_rules ! impl_vec_ld { ($ fun : ident $ fun_lru : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (lvx))] pub unsafe fn $ fun (off : isize , p : * const $ ty) -> t_t_l ! ($ ty) { let addr = (p as * const i8) . offset (off) ; transmute (lvx (addr)) } # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (lvxl))] pub unsafe fn $ fun_lru (off : isize , p : * const $ ty) -> t_t_l ! ($ ty) { let addr = (p as * const i8) . offset (off) ; transmute (lvxl (addr)) } # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorLd for * const $ ty { type Result = t_t_l ! ($ ty) ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_ld (self , off : isize) -> Self :: Result { $ fun (off , self) } # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_ldl (self , off : isize) -> Self :: Result { $ fun_lru (off , self) } } } ; }}
mkitem!{impl_vec_ld ! { vec_ld_u8 vec_ldl_u8 u8 }}
mkitem!{impl_vec_ld ! { vec_ld_i8 vec_ldl_i8 i8 }}
mkitem!{impl_vec_ld ! { vec_ld_u16 vec_ldl_u16 u16 }}
mkitem!{impl_vec_ld ! { vec_ld_i16 vec_ldl_i16 i16 }}
mkitem!{impl_vec_ld ! { vec_ld_u32 vec_ldl_u32 u32 }}
mkitem!{impl_vec_ld ! { vec_ld_i32 vec_ldl_i32 i32 }}
mkitem!{impl_vec_ld ! { vec_ld_f32 vec_ldl_f32 f32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorLde { type Result ; unsafe fn vec_lde (self , a : isize) -> Self :: Result ; }}}
mkitem!{macro_rules ! impl_vec_lde { ($ fun : ident $ instr : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ($ instr))] pub unsafe fn $ fun (a : isize , b : * const $ ty) -> t_t_l ! ($ ty) { let addr = b . byte_offset (a) . cast ::< i8 > () ; transmute ($ instr (addr)) } # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorLde for * const $ ty { type Result = t_t_l ! ($ ty) ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_lde (self , a : isize) -> Self :: Result { $ fun (a , self) } } } ; }}
mkitem!{impl_vec_lde ! { vec_lde_u8 lvebx u8 }}
mkitem!{impl_vec_lde ! { vec_lde_i8 lvebx i8 }}
mkitem!{impl_vec_lde ! { vec_lde_u16 lvehx u16 }}
mkitem!{impl_vec_lde ! { vec_lde_i16 lvehx i16 }}
mkitem!{impl_vec_lde ! { vec_lde_u32 lvewx u32 }}
mkitem!{impl_vec_lde ! { vec_lde_i32 lvewx i32 }}
mkitem!{impl_vec_lde ! { vec_lde_f32 lvewx f32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSt { type Target ; unsafe fn vec_st (self , off : isize , p : Self :: Target) ; unsafe fn vec_stl (self , off : isize , p : Self :: Target) ; }}}
mkitem!{macro_rules ! impl_vec_st { ($ fun : ident $ fun_lru : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (stvx))] pub unsafe fn $ fun (a : t_t_l ! ($ ty) , off : isize , p : * const $ ty) { let addr = (p as * const i8) . offset (off) ; stvx (transmute (a) , addr) } # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (stvxl))] pub unsafe fn $ fun_lru (a : t_t_l ! ($ ty) , off : isize , p : * const $ ty) { let addr = (p as * const i8) . offset (off as isize) ; stvxl (transmute (a) , addr) } # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSt for t_t_l ! ($ ty) { type Target = * const $ ty ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_st (self , off : isize , p : Self :: Target) { $ fun (self , off , p) } # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_stl (self , off : isize , p : Self :: Target) { $ fun (self , off , p) } } } ; }}
mkitem!{impl_vec_st ! { vec_st_u8 vec_stl_u8 u8 }}
mkitem!{impl_vec_st ! { vec_st_i8 vec_stl_i8 i8 }}
mkitem!{impl_vec_st ! { vec_st_u16 vec_stl_u16 u16 }}
mkitem!{impl_vec_st ! { vec_st_i16 vec_stl_i16 i16 }}
mkitem!{impl_vec_st ! { vec_st_u32 vec_stl_u32 u32 }}
mkitem!{impl_vec_st ! { vec_st_i32 vec_stl_i32 i32 }}
mkitem!{impl_vec_st ! { vec_st_f32 vec_stl_f32 f32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSte { type Target ; unsafe fn vec_ste (self , off : isize , p : Self :: Target) ; }}}
mkitem!{macro_rules ! impl_vec_ste { ($ fun : ident $ instr : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ($ instr))] pub unsafe fn $ fun (a : t_t_l ! ($ ty) , off : isize , p : * const $ ty) { let addr = (p as * const i8) . offset (off) ; $ instr (transmute (a) , addr) } # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSte for t_t_l ! ($ ty) { type Target = * const $ ty ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_ste (self , off : isize , p : Self :: Target) { $ fun (self , off , p) } } } ; }}
mkitem!{impl_vec_ste ! { vec_ste_u8 stvebx u8 }}
mkitem!{impl_vec_ste ! { vec_ste_i8 stvebx i8 }}
mkitem!{impl_vec_ste ! { vec_ste_u16 stvehx u16 }}
mkitem!{impl_vec_ste ! { vec_ste_i16 stvehx i16 }}
mkitem!{impl_vec_ste ! { vec_ste_u32 stvewx u32 }}
mkitem!{impl_vec_ste ! { vec_ste_i32 stvewx i32 }}
mkitem!{impl_vec_ste ! { vec_ste_f32 stvewx f32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorXl { type Result ; unsafe fn vec_xl (self , a : isize) -> Self :: Result ; }}}
mkitem!{macro_rules ! impl_vec_xl { ($ fun : ident $ notpwr9 : ident / $ pwr9 : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "power9-altivec")) , assert_instr ($ notpwr9))] # [cfg_attr (all (test , target_feature = "power9-altivec") , assert_instr ($ pwr9))] pub unsafe fn $ fun (a : isize , b : * const $ ty) -> t_t_l ! ($ ty) { let addr = (b as * const u8) . offset (a) ; let mut r = mem :: MaybeUninit :: uninit () ; crate :: ptr :: copy_nonoverlapping (addr , r . as_mut_ptr () as * mut u8 , mem :: size_of ::< t_t_l ! ($ ty) > () ,) ; r . assume_init () } # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorXl for * const $ ty { type Result = t_t_l ! ($ ty) ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_xl (self , a : isize) -> Self :: Result { $ fun (a , self) } } } ; }}
mkitem!{impl_vec_xl ! { vec_xl_i8 lxvd2x / lxv i8 }}
mkitem!{impl_vec_xl ! { vec_xl_u8 lxvd2x / lxv u8 }}
mkitem!{impl_vec_xl ! { vec_xl_i16 lxvd2x / lxv i16 }}
mkitem!{impl_vec_xl ! { vec_xl_u16 lxvd2x / lxv u16 }}
mkitem!{impl_vec_xl ! { vec_xl_i32 lxvd2x / lxv i32 }}
mkitem!{impl_vec_xl ! { vec_xl_u32 lxvd2x / lxv u32 }}
mkitem!{impl_vec_xl ! { vec_xl_f32 lxvd2x / lxv f32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorXst { type Out ; unsafe fn vec_xst (self , a : isize , p : Self :: Out) ; }}}
mkitem!{macro_rules ! impl_vec_xst { ($ fun : ident $ notpwr9 : ident / $ pwr9 : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "power9-altivec")) , assert_instr ($ notpwr9))] # [cfg_attr (all (test , target_feature = "power9-altivec") , assert_instr ($ pwr9))] pub unsafe fn $ fun (s : t_t_l ! ($ ty) , a : isize , b : * mut $ ty) { let addr = (b as * mut u8) . offset (a) ; crate :: ptr :: copy_nonoverlapping (& s as * const _ as * const u8 , addr , mem :: size_of ::< t_t_l ! ($ ty) > () ,) ; } # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorXst for t_t_l ! ($ ty) { type Out = * mut $ ty ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_xst (self , a : isize , b : Self :: Out) { $ fun (self , a , b) } } } ; }}
mkitem!{impl_vec_xst ! { vec_xst_i8 stxvd2x / stxv i8 }}
mkitem!{impl_vec_xst ! { vec_xst_u8 stxvd2x / stxv u8 }}
mkitem!{impl_vec_xst ! { vec_xst_i16 stxvd2x / stxv i16 }}
mkitem!{impl_vec_xst ! { vec_xst_u16 stxvd2x / stxv u16 }}
mkitem!{impl_vec_xst ! { vec_xst_i32 stxvd2x / stxv i32 }}
mkitem!{impl_vec_xst ! { vec_xst_u32 stxvd2x / stxv u32 }}
mkitem!{impl_vec_xst ! { vec_xst_f32 stxvd2x / stxv f32 }}
mkitem!{test_impl ! { vec_floor (a : vector_float) -> vector_float [simd_floor , vrfim / xvrspim] }}
mkitem!{test_impl ! { vec_vexptefp (a : vector_float) -> vector_float [vexptefp , vexptefp] }}
mkitem!{test_impl ! { vec_vcmpgtub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_bool_char [vcmpgtub , vcmpgtub] }}
mkitem!{test_impl ! { vec_vcmpgtuh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_bool_short [vcmpgtuh , vcmpgtuh] }}
mkitem!{test_impl ! { vec_vcmpgtuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_bool_int [vcmpgtuw , vcmpgtuw] }}
mkitem!{test_impl ! { vec_vcmpgtsb (a : vector_signed_char , b : vector_signed_char) -> vector_bool_char [vcmpgtsb , vcmpgtsb] }}
mkitem!{test_impl ! { vec_vcmpgtsh (a : vector_signed_short , b : vector_signed_short) -> vector_bool_short [vcmpgtsh , vcmpgtsh] }}
mkitem!{test_impl ! { vec_vcmpgtsw (a : vector_signed_int , b : vector_signed_int) -> vector_bool_int [vcmpgtsw , vcmpgtsw] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorCmpGt < Other > { type Result ; unsafe fn vec_cmpgt (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_cmp ! { [VectorCmpGt vec_cmpgt] (vec_vcmpgtub , vec_vcmpgtsb , vec_vcmpgtuh , vec_vcmpgtsh , vec_vcmpgtuw , vec_vcmpgtsw) }}
mkitem!{test_impl ! { vec_vcmpgefp (a : vector_float , b : vector_float) -> vector_bool_int [vcmpgefp , vcmpgefp] }}
mkitem!{test_impl ! { vec_vcmpequb (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_bool_char [vcmpequb , vcmpequb] }}
mkitem!{test_impl ! { vec_vcmpequh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_bool_short [vcmpequh , vcmpequh] }}
mkitem!{test_impl ! { vec_vcmpequw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_bool_int [vcmpequw , vcmpequw] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorCmpEq < Other > { type Result ; unsafe fn vec_cmpeq (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_cmp ! { [VectorCmpEq vec_cmpeq] (vec_vcmpequb , vec_vcmpequh , vec_vcmpequw) }}
mkitem!{macro_rules ! impl_cmpne { ($ fun : ident ($ ty : ident) -> $ r : ident $ ([$ pwr9 : ident]) ?) => { # [inline] # [target_feature (enable = "altivec")] $ (# [cfg_attr (all (test , target_feature = "power9-altivec") , assert_instr ($ pwr9))]) ? unsafe fn $ fun (a : $ ty , b : $ ty) -> $ r { $ (if cfg ! (target_feature = "power9-altivec") { transmute ($ pwr9 (transmute (a) , transmute (b))) } else) ? { let zero = transmute (i32x4 :: new (0 , 0 , 0 , 0)) ; vec_nor (vec_cmpeq (a , b) , zero) } } } ; }}
mkitem!{impl_cmpne ! { vec_vcmpneb (vector_signed_char) -> vector_bool_char [vcmpneb] }}
mkitem!{impl_cmpne ! { vec_vcmpneh (vector_signed_short) -> vector_bool_short [vcmpneh] }}
mkitem!{impl_cmpne ! { vec_vcmpnew (vector_signed_int) -> vector_bool_int [vcmpnew] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorCmpNe < Other > { type Result ; unsafe fn vec_cmpne (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_cmp ! { [VectorCmpNe vec_cmpne] (vec_vcmpneb , vec_vcmpneh , vec_vcmpnew) }}
mkitem!{test_impl ! { vec_vcmpbfp (a : vector_float , b : vector_float) -> vector_signed_int [vcmpbfp , vcmpbfp] }}

macro_rules! vcmpequb_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpequb_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpequb_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequb .))] unsafe fn vcmpequb_all (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpequb_p (2 , a , b) != 0 }
}

macro_rules! vcmpequb_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpequb_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpequb_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequb .))] unsafe fn vcmpequb_any (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpequb_p (1 , a , b) != 0 }
}

macro_rules! vcmpequh_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpequh_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpequh_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequh .))] unsafe fn vcmpequh_all (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpequh_p (2 , a , b) != 0 }
}

macro_rules! vcmpequh_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpequh_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpequh_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequh .))] unsafe fn vcmpequh_any (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpequh_p (1 , a , b) != 0 }
}

macro_rules! vcmpequw_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpequw_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpequw_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequw .))] unsafe fn vcmpequw_all (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpequw_p (2 , a , b) != 0 }
}

macro_rules! vcmpequw_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpequw_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpequw_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequw .))] unsafe fn vcmpequw_any (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpequw_p (1 , a , b) != 0 }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAllEq < Other > { type Result ; unsafe fn vec_all_eq (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAllEq vec_all_eq] (vcmpequb_all , vcmpequh_all , vcmpequw_all) }}

macro_rules! vcmpeqfp_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpeqfp_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpeqfp_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpeqfp .))] unsafe fn vcmpeqfp_all (a : vector_float , b : vector_float) -> bool { vcmpeqfp_p (2 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAllEq < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_all_eq (self , b : vector_float) -> Self :: Result { vcmpeqfp_all (self , b) } }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAnyEq < Other > { type Result ; unsafe fn vec_any_eq (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAnyEq vec_any_eq] (vcmpequb_any , vcmpequh_any , vcmpequw_any) }}

macro_rules! vcmpeqfp_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpeqfp_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpeqfp_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpeqfp .))] unsafe fn vcmpeqfp_any (a : vector_float , b : vector_float) -> bool { vcmpeqfp_p (1 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAnyEq < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_any_eq (self , b : vector_float) -> Self :: Result { vcmpeqfp_any (self , b) } }}}

macro_rules! vcmpgesb_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgesb_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgesb_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsb .))] unsafe fn vcmpgesb_all (a : vector_signed_char , b : vector_signed_char) -> bool { vcmpgtsb_p (0 , b , a) != 0 }
}

macro_rules! vcmpgesb_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgesb_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgesb_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsb .))] unsafe fn vcmpgesb_any (a : vector_signed_char , b : vector_signed_char) -> bool { vcmpgtsb_p (3 , b , a) != 0 }
}

macro_rules! vcmpgesh_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgesh_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgesh_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsh .))] unsafe fn vcmpgesh_all (a : vector_signed_short , b : vector_signed_short) -> bool { vcmpgtsh_p (0 , b , a) != 0 }
}

macro_rules! vcmpgesh_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgesh_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgesh_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsh .))] unsafe fn vcmpgesh_any (a : vector_signed_short , b : vector_signed_short) -> bool { vcmpgtsh_p (3 , b , a) != 0 }
}

macro_rules! vcmpgesw_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgesw_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgesw_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsw .))] unsafe fn vcmpgesw_all (a : vector_signed_int , b : vector_signed_int) -> bool { vcmpgtsw_p (0 , b , a) != 0 }
}

macro_rules! vcmpgesw_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgesw_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgesw_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsw .))] unsafe fn vcmpgesw_any (a : vector_signed_int , b : vector_signed_int) -> bool { vcmpgtsw_p (3 , b , a) != 0 }
}

macro_rules! vcmpgeub_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgeub_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgeub_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtub .))] unsafe fn vcmpgeub_all (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpgtub_p (0 , b , a) != 0 }
}

macro_rules! vcmpgeub_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgeub_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgeub_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtub .))] unsafe fn vcmpgeub_any (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpgtub_p (3 , b , a) != 0 }
}

macro_rules! vcmpgeuh_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgeuh_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgeuh_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuh .))] unsafe fn vcmpgeuh_all (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpgtuh_p (0 , b , a) != 0 }
}

macro_rules! vcmpgeuh_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgeuh_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgeuh_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuh .))] unsafe fn vcmpgeuh_any (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpgtuh_p (3 , b , a) != 0 }
}

macro_rules! vcmpgeuw_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgeuw_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgeuw_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuw .))] unsafe fn vcmpgeuw_all (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpgtuw_p (0 , b , a) != 0 }
}

macro_rules! vcmpgeuw_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgeuw_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgeuw_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuw .))] unsafe fn vcmpgeuw_any (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpgtuw_p (3 , b , a) != 0 }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAllGe < Other > { type Result ; unsafe fn vec_all_ge (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAllGe vec_all_ge] (vcmpgeub_all , vcmpgesb_all , vcmpgeuh_all , vcmpgesh_all , vcmpgeuw_all , vcmpgesw_all) }}

macro_rules! vcmpgefp_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgefp_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgefp_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgefp .))] unsafe fn vcmpgefp_all (a : vector_float , b : vector_float) -> bool { vcmpgefp_p (2 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAllGe < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_all_ge (self , b : vector_float) -> Self :: Result { vcmpgefp_all (self , b) } }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAnyGe < Other > { type Result ; unsafe fn vec_any_ge (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAnyGe vec_any_ge] (vcmpgeub_any , vcmpgesb_any , vcmpgeuh_any , vcmpgesh_any , vcmpgeuw_any , vcmpgesw_any) }}

macro_rules! vcmpgefp_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgefp_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgefp_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgefp .))] unsafe fn vcmpgefp_any (a : vector_float , b : vector_float) -> bool { vcmpgefp_p (1 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAnyGe < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_any_ge (self , b : vector_float) -> Self :: Result { vcmpgefp_any (self , b) } }}}

macro_rules! vcmpgtsb_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtsb_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtsb_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsb .))] unsafe fn vcmpgtsb_all (a : vector_signed_char , b : vector_signed_char) -> bool { vcmpgtsb_p (2 , a , b) != 0 }
}

macro_rules! vcmpgtsb_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtsb_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtsb_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsb .))] unsafe fn vcmpgtsb_any (a : vector_signed_char , b : vector_signed_char) -> bool { vcmpgtsb_p (1 , a , b) != 0 }
}

macro_rules! vcmpgtsh_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtsh_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtsh_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsh .))] unsafe fn vcmpgtsh_all (a : vector_signed_short , b : vector_signed_short) -> bool { vcmpgtsh_p (2 , a , b) != 0 }
}

macro_rules! vcmpgtsh_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtsh_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtsh_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsh .))] unsafe fn vcmpgtsh_any (a : vector_signed_short , b : vector_signed_short) -> bool { vcmpgtsh_p (1 , a , b) != 0 }
}

macro_rules! vcmpgtsw_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtsw_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtsw_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsw .))] unsafe fn vcmpgtsw_all (a : vector_signed_int , b : vector_signed_int) -> bool { vcmpgtsw_p (2 , a , b) != 0 }
}

macro_rules! vcmpgtsw_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtsw_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtsw_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtsw .))] unsafe fn vcmpgtsw_any (a : vector_signed_int , b : vector_signed_int) -> bool { vcmpgtsw_p (1 , a , b) != 0 }
}

macro_rules! vcmpgtub_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtub_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtub_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtub .))] unsafe fn vcmpgtub_all (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpgtub_p (2 , a , b) != 0 }
}

macro_rules! vcmpgtub_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtub_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtub_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtub .))] unsafe fn vcmpgtub_any (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpgtub_p (1 , a , b) != 0 }
}

macro_rules! vcmpgtuh_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtuh_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtuh_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuh .))] unsafe fn vcmpgtuh_all (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpgtuh_p (2 , a , b) != 0 }
}

macro_rules! vcmpgtuh_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtuh_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtuh_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuh .))] unsafe fn vcmpgtuh_any (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpgtuh_p (1 , a , b) != 0 }
}

macro_rules! vcmpgtuw_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtuw_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtuw_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuw .))] unsafe fn vcmpgtuw_all (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpgtuw_p (2 , a , b) != 0 }
}

macro_rules! vcmpgtuw_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtuw_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtuw_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtuw .))] unsafe fn vcmpgtuw_any (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpgtuw_p (1 , a , b) != 0 }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAllGt < Other > { type Result ; unsafe fn vec_all_gt (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAllGt vec_all_gt] (vcmpgtub_all , vcmpgtsb_all , vcmpgtuh_all , vcmpgtsh_all , vcmpgtuw_all , vcmpgtsw_all) }}

macro_rules! vcmpgtfp_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtfp_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtfp_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtfp .))] unsafe fn vcmpgtfp_all (a : vector_float , b : vector_float) -> bool { vcmpgtfp_p (2 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAllGt < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_all_gt (self , b : vector_float) -> Self :: Result { vcmpgtfp_all (self , b) } }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAnyGt < Other > { type Result ; unsafe fn vec_any_gt (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAnyGt vec_any_gt] (vcmpgtub_any , vcmpgtsb_any , vcmpgtuh_any , vcmpgtsh_any , vcmpgtuw_any , vcmpgtsw_any) }}

macro_rules! vcmpgtfp_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpgtfp_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpgtfp_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpgtfp .))] unsafe fn vcmpgtfp_any (a : vector_float , b : vector_float) -> bool { vcmpgtfp_p (1 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAnyGt < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_any_gt (self , b : vector_float) -> Self :: Result { vcmpgtfp_any (self , b) } }}}

macro_rules! vcmpneub_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpneub_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpneub_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequb .))] unsafe fn vcmpneub_all (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpequb_p (0 , a , b) != 0 }
}

macro_rules! vcmpneub_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpneub_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpneub_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequb .))] unsafe fn vcmpneub_any (a : vector_unsigned_char , b : vector_unsigned_char) -> bool { vcmpequb_p (3 , a , b) != 0 }
}

macro_rules! vcmpneuh_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpneuh_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpneuh_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequh .))] unsafe fn vcmpneuh_all (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpequh_p (0 , a , b) != 0 }
}

macro_rules! vcmpneuh_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpneuh_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpneuh_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequh .))] unsafe fn vcmpneuh_any (a : vector_unsigned_short , b : vector_unsigned_short) -> bool { vcmpequh_p (3 , a , b) != 0 }
}

macro_rules! vcmpneuw_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpneuw_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpneuw_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequw .))] unsafe fn vcmpneuw_all (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpequw_p (0 , a , b) != 0 }
}

macro_rules! vcmpneuw_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpneuw_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpneuw_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpequw .))] unsafe fn vcmpneuw_any (a : vector_unsigned_int , b : vector_unsigned_int) -> bool { vcmpequw_p (3 , a , b) != 0 }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAllNe < Other > { type Result ; unsafe fn vec_all_ne (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAllNe vec_all_ne] (vcmpneub_all , vcmpneuh_all , vcmpneuw_all) }}

macro_rules! vcmpnefp_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpnefp_all in module {}", module_path!());
    };
}

mkfn!{
    vcmpnefp_all_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpeqfp .))] unsafe fn vcmpnefp_all (a : vector_float , b : vector_float) -> bool { vcmpeqfp_p (0 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAllNe < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_all_ne (self , b : vector_float) -> Self :: Result { vcmpnefp_all (self , b) } }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAnyNe < Other > { type Result ; unsafe fn vec_any_ne (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_any_all ! { [VectorAnyNe vec_any_ne] (vcmpneub_any , vcmpneuh_any , vcmpneuw_any) }}

macro_rules! vcmpnefp_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vcmpnefp_any in module {}", module_path!());
    };
}

mkfn!{
    vcmpnefp_any_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcmpeqfp .))] unsafe fn vcmpnefp_any (a : vector_float , b : vector_float) -> bool { vcmpeqfp_p (3 , a , b) != 0 }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAnyNe < vector_float > for vector_float { type Result = bool ; # [inline] unsafe fn vec_any_ne (self , b : vector_float) -> Self :: Result { vcmpnefp_any (self , b) } }}}
mkitem!{test_impl ! { vec_vceil (a : vector_float) -> vector_float [simd_ceil , vrfip / xvrspip] }}
mkitem!{test_impl ! { vec_vavgsb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char [vavgsb , vavgsb] }}
mkitem!{test_impl ! { vec_vavgsh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short [vavgsh , vavgsh] }}
mkitem!{test_impl ! { vec_vavgsw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int [vavgsw , vavgsw] }}
mkitem!{test_impl ! { vec_vavgub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char [vavgub , vavgub] }}
mkitem!{test_impl ! { vec_vavguh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short [vavguh , vavguh] }}
mkitem!{test_impl ! { vec_vavguw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [vavguw , vavguw] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAvg < Other > { type Result ; unsafe fn vec_avg (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorAvg vec_avg] 2 (vec_vavgub , vec_vavgsb , vec_vavguh , vec_vavgsh , vec_vavguw , vec_vavgsw) }}

macro_rules! andc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function andc in module {}", module_path!());
    };
}

mkfn!{
    andc_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "vsx")) , assert_instr (vandc))] # [cfg_attr (all (test , target_feature = "vsx") , assert_instr (xxlandc))] unsafe fn andc (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char { let a = transmute (a) ; let b = transmute (b) ; transmute (simd_and (simd_xor (u8x16 :: splat (0xff) , b) , a)) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAndc < Other > { type Result ; unsafe fn vec_andc (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorAndc vec_andc] + 2b (andc) }}

macro_rules! orc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function orc in module {}", module_path!());
    };
}

mkfn!{
    orc_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "vsx")) , assert_instr (vorc))] # [cfg_attr (all (test , target_feature = "vsx") , assert_instr (xxlorc))] unsafe fn orc (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char { let a = transmute (a) ; let b = transmute (b) ; transmute (simd_or (simd_xor (u8x16 :: splat (0xff) , b) , a)) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorOrc < Other > { type Result ; unsafe fn vec_orc (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorOrc vec_orc] + 2b (orc) }}
mkitem!{test_impl ! { vec_vand (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char [simd_and , vand / xxland] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAnd < Other > { type Result ; unsafe fn vec_and (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorAnd vec_and] ~ (simd_and) }}
mkitem!{test_impl ! { vec_vaddsbs (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char [vaddsbs , vaddsbs] }}
mkitem!{test_impl ! { vec_vaddshs (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short [vaddshs , vaddshs] }}
mkitem!{test_impl ! { vec_vaddsws (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int [vaddsws , vaddsws] }}
mkitem!{test_impl ! { vec_vaddubs (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char [vaddubs , vaddubs] }}
mkitem!{test_impl ! { vec_vadduhs (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short [vadduhs , vadduhs] }}
mkitem!{test_impl ! { vec_vadduws (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [vadduws , vadduws] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAdds < Other > { type Result ; unsafe fn vec_adds (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorAdds vec_adds] ~ (vaddubs , vaddsbs , vadduhs , vaddshs , vadduws , vaddsws) }}
mkitem!{test_impl ! { vec_vaddcuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [vaddcuw , vaddcuw] }}
mkitem!{test_impl ! { vec_vsubsbs (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char [vsubsbs , vsubsbs] }}
mkitem!{test_impl ! { vec_vsubshs (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short [vsubshs , vsubshs] }}
mkitem!{test_impl ! { vec_vsubsws (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int [vsubsws , vsubsws] }}
mkitem!{test_impl ! { vec_vsububs (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char [vsububs , vsububs] }}
mkitem!{test_impl ! { vec_vsubuhs (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short [vsubuhs , vsubuhs] }}
mkitem!{test_impl ! { vec_vsubuws (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [vsubuws , vsubuws] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSubs < Other > { type Result ; unsafe fn vec_subs (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorSubs vec_subs] ~ (vsububs , vsubsbs , vsubuhs , vsubshs , vsubuws , vsubsws) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAbs { unsafe fn vec_abs (self) -> Self ; }}}
mkitem!{macro_rules ! impl_abs { ($ name : ident , $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] unsafe fn $ name (v : s_t_l ! ($ ty)) -> s_t_l ! ($ ty) { v . vec_max (- v) } impl_vec_trait ! { [VectorAbs vec_abs] $ name (s_t_l ! ($ ty)) } } ; }}
mkitem!{impl_abs ! { vec_abs_i8 , i8x16 }}
mkitem!{impl_abs ! { vec_abs_i16 , i16x8 }}
mkitem!{impl_abs ! { vec_abs_i32 , i32x4 }}

macro_rules! vec_abs_f32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_abs_f32 in module {}", module_path!());
    };
}

mkfn!{
    vec_abs_f32_introspect!();
    # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_abs_f32 (v : vector_float) -> vector_float { let v : u32x4 = transmute (v) ; transmute (simd_and (v , u32x4 :: splat (0x7FFFFFFF))) }
}
mkitem!{impl_vec_trait ! { [VectorAbs vec_abs] vec_abs_f32 (vector_float) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAbss { unsafe fn vec_abss (self) -> Self ; }}}
mkitem!{macro_rules ! impl_abss { ($ name : ident , $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] unsafe fn $ name (v : s_t_l ! ($ ty)) -> s_t_l ! ($ ty) { let zero : s_t_l ! ($ ty) = transmute (0u8 . vec_splats ()) ; v . vec_max (zero . vec_subs (v)) } impl_vec_trait ! { [VectorAbss vec_abss] $ name (s_t_l ! ($ ty)) } } ; }}
mkitem!{impl_abss ! { vec_abss_i8 , i8x16 }}
mkitem!{impl_abss ! { vec_abss_i16 , i16x8 }}
mkitem!{impl_abss ! { vec_abss_i32 , i32x4 }}

macro_rules! vspltb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vspltb in module {}", module_path!());
    };
}

mkfn!{
    vspltb_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vspltb , IMM4 = 15))] unsafe fn vspltb < const IMM4 : u32 > (a : vector_signed_char) -> vector_signed_char { static_assert_uimm_bits ! (IMM4 , 4) ; simd_shuffle (a , a , const { u32x16 :: from_array ([IMM4 ; 16]) }) }
}

macro_rules! vsplth_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vsplth in module {}", module_path!());
    };
}

mkfn!{
    vsplth_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vsplth , IMM3 = 7))] unsafe fn vsplth < const IMM3 : u32 > (a : vector_signed_short) -> vector_signed_short { static_assert_uimm_bits ! (IMM3 , 3) ; simd_shuffle (a , a , const { u32x8 :: from_array ([IMM3 ; 8]) }) }
}

macro_rules! vspltw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vspltw in module {}", module_path!());
    };
}

mkfn!{
    vspltw_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "vsx")) , assert_instr (vspltw , IMM2 = 3))] # [cfg_attr (all (test , target_feature = "vsx") , assert_instr (xxspltw , IMM2 = 3))] unsafe fn vspltw < const IMM2 : u32 > (a : vector_signed_int) -> vector_signed_int { static_assert_uimm_bits ! (IMM2 , 2) ; simd_shuffle (a , a , const { u32x4 :: from_array ([IMM2 ; 4]) }) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSplat { unsafe fn vec_splat < const IMM : u32 > (self) -> Self ; }}}
mkitem!{macro_rules ! impl_vec_splat { ($ ty : ty , $ fun : ident) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSplat for $ ty { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_splat < const IMM : u32 > (self) -> Self { transmute ($ fun ::< IMM > (transmute (self))) } } } ; }}
mkitem!{impl_vec_splat ! { vector_signed_char , vspltb }}
mkitem!{impl_vec_splat ! { vector_unsigned_char , vspltb }}
mkitem!{impl_vec_splat ! { vector_bool_char , vspltb }}
mkitem!{impl_vec_splat ! { vector_signed_short , vsplth }}
mkitem!{impl_vec_splat ! { vector_unsigned_short , vsplth }}
mkitem!{impl_vec_splat ! { vector_bool_short , vsplth }}
mkitem!{impl_vec_splat ! { vector_signed_int , vspltw }}
mkitem!{impl_vec_splat ! { vector_unsigned_int , vspltw }}
mkitem!{impl_vec_splat ! { vector_bool_int , vspltw }}
mkitem!{macro_rules ! splat { ($ name : ident , $ v : ident , $ r : ident [$ instr_altivec : ident / $ instr_pwr9 : ident , $ doc : literal]) => { # [doc = $ doc] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "vsx")) , assert_instr ($ instr_altivec , IMM5 = 1))] # [cfg_attr (all (test , target_feature = "power9-vector") , assert_instr ($ instr_pwr9 , IMM5 = 1))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn $ name < const IMM5 : i8 > () -> s_t_l ! ($ r) { static_assert_simm_bits ! (IMM5 , 5) ; transmute ($ r :: splat (IMM5 as $ v)) } } ; ($ name : ident , $ v : ident , $ r : ident [$ instr : ident , $ doc : literal]) => { splat ! { $ name , $ v , $ r [$ instr / $ instr , $ doc] } } ; }}
mkitem!{macro_rules ! splats { ($ name : ident , $ v : ident , $ r : ident) => { # [inline] # [target_feature (enable = "altivec")] unsafe fn $ name (v : $ v) -> s_t_l ! ($ r) { transmute ($ r :: splat (v)) } } ; }}
mkitem!{splats ! { splats_u8 , u8 , u8x16 }}
mkitem!{splats ! { splats_u16 , u16 , u16x8 }}
mkitem!{splats ! { splats_u32 , u32 , u32x4 }}
mkitem!{splats ! { splats_i8 , i8 , i8x16 }}
mkitem!{splats ! { splats_i16 , i16 , i16x8 }}
mkitem!{splats ! { splats_i32 , i32 , i32x4 }}
mkitem!{splats ! { splats_f32 , f32 , f32x4 }}
mkitem!{test_impl ! { vec_splats_u8 (v : u8) -> vector_unsigned_char [splats_u8 , vspltb] }}
mkitem!{test_impl ! { vec_splats_u16 (v : u16) -> vector_unsigned_short [splats_u16 , vsplth] }}
mkitem!{test_impl ! { vec_splats_u32 (v : u32) -> vector_unsigned_int [splats_u32 , vspltw / xxspltw / mtvsrws] }}
mkitem!{test_impl ! { vec_splats_i8 (v : i8) -> vector_signed_char [splats_i8 , vspltb] }}
mkitem!{test_impl ! { vec_splats_i16 (v : i16) -> vector_signed_short [splats_i16 , vsplth] }}
mkitem!{test_impl ! { vec_splats_i32 (v : i32) -> vector_signed_int [splats_i32 , vspltw / xxspltw / mtvsrws] }}
mkitem!{test_impl ! { vec_splats_f32 (v : f32) -> vector_float [splats_f32 , vspltw / xxspltw / mtvsrws] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSplats { type Result ; unsafe fn vec_splats (self) -> Self :: Result ; }}}
mkitem!{macro_rules ! impl_vec_splats { ($ (($ fn : ident ($ ty : ty) -> $ r : ty)) ,*) => { $ (impl_vec_trait ! { [VectorSplats vec_splats] $ fn ($ ty) -> $ r }) * } }}
mkitem!{impl_vec_splats ! { (vec_splats_u8 (u8) -> vector_unsigned_char) , (vec_splats_i8 (i8) -> vector_signed_char) , (vec_splats_u16 (u16) -> vector_unsigned_short) , (vec_splats_i16 (i16) -> vector_signed_short) , (vec_splats_u32 (u32) -> vector_unsigned_int) , (vec_splats_i32 (i32) -> vector_signed_int) , (vec_splats_f32 (f32) -> vector_float) }}
mkitem!{test_impl ! { vec_vsububm (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char [simd_sub , vsububm] }}
mkitem!{test_impl ! { vec_vsubuhm (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short [simd_sub , vsubuhm] }}
mkitem!{test_impl ! { vec_vsubuwm (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [simd_sub , vsubuwm] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSub < Other > { type Result ; unsafe fn vec_sub (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorSub vec_sub] ~ (simd_sub , simd_sub , simd_sub , simd_sub , simd_sub , simd_sub) }}
mkitem!{impl_vec_trait ! { [VectorSub vec_sub] simd_sub (vector_float , vector_float) -> vector_float }}
mkitem!{test_impl ! { vec_vsubcuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [vsubcuw , vsubcuw] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSubc < Other > { type Result ; unsafe fn vec_subc (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorSubc vec_subc] + vec_vsubcuw (vector_unsigned_int , vector_unsigned_int) -> vector_unsigned_int }}
mkitem!{impl_vec_trait ! { [VectorSubc vec_subc] + vec_vsubcuw (vector_signed_int , vector_signed_int) -> vector_signed_int }}
mkitem!{test_impl ! { vec_vminsb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char [vminsb , vminsb] }}
mkitem!{test_impl ! { vec_vminsh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short [vminsh , vminsh] }}
mkitem!{test_impl ! { vec_vminsw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int [vminsw , vminsw] }}
mkitem!{test_impl ! { vec_vminub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char [vminub , vminub] }}
mkitem!{test_impl ! { vec_vminuh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short [vminuh , vminuh] }}
mkitem!{test_impl ! { vec_vminuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [vminuw , vminuw] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMin < Other > { type Result ; unsafe fn vec_min (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorMin vec_min] ~ (vminub , vminsb , vminuh , vminsh , vminuw , vminsw) }}
mkitem!{test_impl ! { vec_vmaxsb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char [vmaxsb , vmaxsb] }}
mkitem!{test_impl ! { vec_vmaxsh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short [vmaxsh , vmaxsh] }}
mkitem!{test_impl ! { vec_vmaxsw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int [vmaxsw , vmaxsw] }}
mkitem!{test_impl ! { vec_vmaxub (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char [vmaxub , vmaxub] }}
mkitem!{test_impl ! { vec_vmaxuh (a : vector_unsigned_short , b : vector_unsigned_short) -> vector_unsigned_short [vmaxuh , vmaxuh] }}
mkitem!{test_impl ! { vec_vmaxuw (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int [vmaxuw , vmaxuw] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMax < Other > { type Result ; unsafe fn vec_max (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorMax vec_max] ~ (vmaxub , vmaxsb , vmaxuh , vmaxsh , vmaxuw , vmaxsw) }}

macro_rules! vec_vmuleub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmuleub in module {}", module_path!());
    };
}

mkfn!{
    vec_vmuleub_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmuleub))] unsafe fn vec_vmuleub (a : vector_unsigned_char , b : vector_unsigned_char ,) -> vector_unsigned_short { vmuleub (a , b) }
}

macro_rules! vec_vmulesb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmulesb in module {}", module_path!());
    };
}

mkfn!{
    vec_vmulesb_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmulesb))] unsafe fn vec_vmulesb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_short { vmulesb (a , b) }
}

macro_rules! vec_vmuleuh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmuleuh in module {}", module_path!());
    };
}

mkfn!{
    vec_vmuleuh_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmuleuh))] unsafe fn vec_vmuleuh (a : vector_unsigned_short , b : vector_unsigned_short ,) -> vector_unsigned_int { vmuleuh (a , b) }
}

macro_rules! vec_vmulesh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmulesh in module {}", module_path!());
    };
}

mkfn!{
    vec_vmulesh_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmulesh))] unsafe fn vec_vmulesh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_int { vmulesh (a , b) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMul { unsafe fn vec_mul (self , b : Self) -> Self ; }}}

macro_rules! vec_vmuluwm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmuluwm in module {}", module_path!());
    };
}

mkfn!{
    vec_vmuluwm_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmuluwm))] unsafe fn vec_vmuluwm (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int { transmute (simd_mul :: < i32x4 > (transmute (a) , transmute (b))) }
}

macro_rules! vec_xvmulsp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_xvmulsp in module {}", module_path!());
    };
}

mkfn!{
    vec_xvmulsp_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (xvmulsp))] unsafe fn vec_xvmulsp (a : vector_float , b : vector_float) -> vector_float { transmute (simd_mul :: < f32x4 > (transmute (a) , transmute (b))) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMul for vector_signed_int { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mul (self , b : Self) -> Self { vec_vmuluwm (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMul for vector_unsigned_int { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mul (self , b : Self) -> Self { transmute (simd_mul :: < u32x4 > (transmute (self) , transmute (b))) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMul for vector_float { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mul (self , b : Self) -> Self { vec_xvmulsp (self , b) } }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMule < Result > { unsafe fn vec_mule (self , b : Self) -> Result ; }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMule < vector_unsigned_short > for vector_unsigned_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mule (self , b : Self) -> vector_unsigned_short { vmuleub (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMule < vector_signed_short > for vector_signed_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mule (self , b : Self) -> vector_signed_short { vmulesb (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMule < vector_unsigned_int > for vector_unsigned_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mule (self , b : Self) -> vector_unsigned_int { vmuleuh (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMule < vector_signed_int > for vector_signed_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mule (self , b : Self) -> vector_signed_int { vmulesh (self , b) } }}}

macro_rules! vec_vmuloub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmuloub in module {}", module_path!());
    };
}

mkfn!{
    vec_vmuloub_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmuloub))] unsafe fn vec_vmuloub (a : vector_unsigned_char , b : vector_unsigned_char ,) -> vector_unsigned_short { vmuloub (a , b) }
}

macro_rules! vec_vmulosb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmulosb in module {}", module_path!());
    };
}

mkfn!{
    vec_vmulosb_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmulosb))] unsafe fn vec_vmulosb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_short { vmulosb (a , b) }
}

macro_rules! vec_vmulouh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmulouh in module {}", module_path!());
    };
}

mkfn!{
    vec_vmulouh_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmulouh))] unsafe fn vec_vmulouh (a : vector_unsigned_short , b : vector_unsigned_short ,) -> vector_unsigned_int { vmulouh (a , b) }
}

macro_rules! vec_vmulosh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmulosh in module {}", module_path!());
    };
}

mkfn!{
    vec_vmulosh_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmulosh))] unsafe fn vec_vmulosh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_int { vmulosh (a , b) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMulo < Result > { unsafe fn vec_mulo (self , b : Self) -> Result ; }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMulo < vector_unsigned_short > for vector_unsigned_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mulo (self , b : Self) -> vector_unsigned_short { vmuloub (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMulo < vector_signed_short > for vector_signed_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mulo (self , b : Self) -> vector_signed_short { vmulosb (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMulo < vector_unsigned_int > for vector_unsigned_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mulo (self , b : Self) -> vector_unsigned_int { vmulouh (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMulo < vector_signed_int > for vector_signed_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mulo (self , b : Self) -> vector_signed_int { vmulosh (self , b) } }}}

macro_rules! vec_vsum4ubs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vsum4ubs in module {}", module_path!());
    };
}

mkfn!{
    vec_vsum4ubs_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vsum4ubs))] unsafe fn vec_vsum4ubs (a : vector_unsigned_char , b : vector_unsigned_int) -> vector_unsigned_int { vsum4ubs (a , b) }
}

macro_rules! vec_vsum4sbs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vsum4sbs in module {}", module_path!());
    };
}

mkfn!{
    vec_vsum4sbs_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vsum4sbs))] unsafe fn vec_vsum4sbs (a : vector_signed_char , b : vector_signed_int) -> vector_signed_int { vsum4sbs (a , b) }
}

macro_rules! vec_vsum4shs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vsum4shs in module {}", module_path!());
    };
}

mkfn!{
    vec_vsum4shs_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vsum4shs))] unsafe fn vec_vsum4shs (a : vector_signed_short , b : vector_signed_int) -> vector_signed_int { vsum4shs (a , b) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSum4s < Other > { unsafe fn vec_sum4s (self , b : Other) -> Other ; }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSum4s < vector_unsigned_int > for vector_unsigned_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_sum4s (self , b : vector_unsigned_int) -> vector_unsigned_int { vsum4ubs (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSum4s < vector_signed_int > for vector_signed_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_sum4s (self , b : vector_signed_int) -> vector_signed_int { vsum4sbs (self , b) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSum4s < vector_signed_int > for vector_signed_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_sum4s (self , b : vector_signed_int) -> vector_signed_int { vsum4shs (self , b) } }}}

macro_rules! vec_vsum2sws_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vsum2sws in module {}", module_path!());
    };
}

mkfn!{
    vec_vsum2sws_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vsum2sws))] unsafe fn vec_vsum2sws (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int { vsum2sws (a , b) }
}

macro_rules! vec_vnmsubfp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vnmsubfp in module {}", module_path!());
    };
}

mkfn!{
    vec_vnmsubfp_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vnmsubfp))] unsafe fn vec_vnmsubfp (a : vector_float , b : vector_float , c : vector_float) -> vector_float { vnmsubfp (a , b , c) }
}

macro_rules! vec_vmaddfp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmaddfp in module {}", module_path!());
    };
}

mkfn!{
    vec_vmaddfp_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (xvmaddasp))] pub unsafe fn vec_vmaddfp (a : vector_float , b : vector_float , c : vector_float) -> vector_float { simd_fma (a , b , c) }
}

macro_rules! vec_vmsumubm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmsumubm in module {}", module_path!());
    };
}

mkfn!{
    vec_vmsumubm_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmsumubm))] unsafe fn vec_vmsumubm (a : vector_unsigned_char , b : vector_unsigned_char , c : vector_unsigned_int ,) -> vector_unsigned_int { vmsumubm (a , b , c) }
}

macro_rules! vec_vmsummbm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmsummbm in module {}", module_path!());
    };
}

mkfn!{
    vec_vmsummbm_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmsummbm))] unsafe fn vec_vmsummbm (a : vector_signed_char , b : vector_unsigned_char , c : vector_signed_int ,) -> vector_signed_int { vmsummbm (a , b , c) }
}

macro_rules! vec_vmsumuhm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmsumuhm in module {}", module_path!());
    };
}

mkfn!{
    vec_vmsumuhm_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmsumuhm))] unsafe fn vec_vmsumuhm (a : vector_unsigned_short , b : vector_unsigned_short , c : vector_unsigned_int ,) -> vector_unsigned_int { vmsumuhm (a , b , c) }
}

macro_rules! vec_vmsumshm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmsumshm in module {}", module_path!());
    };
}

mkfn!{
    vec_vmsumshm_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmsumshm))] unsafe fn vec_vmsumshm (a : vector_signed_short , b : vector_signed_short , c : vector_signed_int ,) -> vector_signed_int { vmsumshm (a , b , c) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMsum < B , Other > { unsafe fn vec_msum (self , b : B , c : Other) -> Other ; }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMsum < vector_unsigned_char , vector_unsigned_int > for vector_unsigned_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_msum (self , b : vector_unsigned_char , c : vector_unsigned_int ,) -> vector_unsigned_int { vmsumubm (self , b , c) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMsum < vector_unsigned_char , vector_signed_int > for vector_signed_char { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_msum (self , b : vector_unsigned_char , c : vector_signed_int ,) -> vector_signed_int { vmsummbm (self , b , c) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMsum < vector_unsigned_short , vector_unsigned_int > for vector_unsigned_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_msum (self , b : vector_unsigned_short , c : vector_unsigned_int ,) -> vector_unsigned_int { vmsumuhm (self , b , c) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMsum < vector_signed_short , vector_signed_int > for vector_signed_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_msum (self , b : vector_signed_short , c : vector_signed_int ,) -> vector_signed_int { vmsumshm (self , b , c) } }}}

macro_rules! vec_vmsumuhs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmsumuhs in module {}", module_path!());
    };
}

mkfn!{
    vec_vmsumuhs_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmsumuhs))] unsafe fn vec_vmsumuhs (a : vector_unsigned_short , b : vector_unsigned_short , c : vector_unsigned_int ,) -> vector_unsigned_int { vmsumuhs (a , b , c) }
}

macro_rules! vec_vmsumshs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmsumshs in module {}", module_path!());
    };
}

mkfn!{
    vec_vmsumshs_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmsumshs))] unsafe fn vec_vmsumshs (a : vector_signed_short , b : vector_signed_short , c : vector_signed_int ,) -> vector_signed_int { vmsumshs (a , b , c) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMsums < Other > { unsafe fn vec_msums (self , b : Self , c : Other) -> Other ; }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMsums < vector_unsigned_int > for vector_unsigned_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_msums (self , b : Self , c : vector_unsigned_int) -> vector_unsigned_int { vmsumuhs (self , b , c) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMsums < vector_signed_int > for vector_signed_short { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_msums (self , b : Self , c : vector_signed_int) -> vector_signed_int { vmsumshs (self , b , c) } }}}

macro_rules! vec_vperm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vperm in module {}", module_path!());
    };
}

mkfn!{
    vec_vperm_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vperm))] unsafe fn vec_vperm (a : vector_signed_int , b : vector_signed_int , c : vector_unsigned_char ,) -> vector_signed_int { vperm (a , b , c) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorPerm { unsafe fn vec_vperm (self , b : Self , c : vector_unsigned_char) -> Self ; }}}
mkitem!{macro_rules ! vector_perm { { $ impl : ident } => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorPerm for $ impl { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_vperm (self , b : Self , c : vector_unsigned_char) -> Self { transmute (vec_vperm (transmute (self) , transmute (b) , c)) } } } }}
mkitem!{vector_perm ! { vector_signed_char }}
mkitem!{vector_perm ! { vector_unsigned_char }}
mkitem!{vector_perm ! { vector_bool_char }}
mkitem!{vector_perm ! { vector_signed_short }}
mkitem!{vector_perm ! { vector_unsigned_short }}
mkitem!{vector_perm ! { vector_bool_short }}
mkitem!{vector_perm ! { vector_signed_int }}
mkitem!{vector_perm ! { vector_unsigned_int }}
mkitem!{vector_perm ! { vector_bool_int }}
mkitem!{vector_perm ! { vector_float }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAdd < Other > { type Result ; unsafe fn vec_add (self , other : Other) -> Self :: Result ; }}}

macro_rules! vec_add_bc_sc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_bc_sc in module {}", module_path!());
    };
}

mkfn!{
    vec_add_bc_sc_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vaddubm))] pub unsafe fn vec_add_bc_sc (a : vector_bool_char , b : vector_signed_char) -> vector_signed_char { simd_add (transmute (a) , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_signed_char > for vector_bool_char { type Result = vector_signed_char ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_signed_char) -> Self :: Result { vec_add_bc_sc (self , other) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_bool_char > for vector_signed_char { type Result = vector_signed_char ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_bool_char) -> Self :: Result { other . vec_add (self) } }}}

macro_rules! vec_add_sc_sc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_sc_sc in module {}", module_path!());
    };
}

mkfn!{
    vec_add_sc_sc_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vaddubm))] pub unsafe fn vec_add_sc_sc (a : vector_signed_char , b : vector_signed_char ,) -> vector_signed_char { simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_signed_char > for vector_signed_char { type Result = vector_signed_char ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_signed_char) -> Self :: Result { vec_add_sc_sc (self , other) } }}}

macro_rules! vec_add_bc_uc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_bc_uc in module {}", module_path!());
    };
}

mkfn!{
    vec_add_bc_uc_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vaddubm))] pub unsafe fn vec_add_bc_uc (a : vector_bool_char , b : vector_unsigned_char ,) -> vector_unsigned_char { simd_add (transmute (a) , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_unsigned_char > for vector_bool_char { type Result = vector_unsigned_char ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_unsigned_char) -> Self :: Result { vec_add_bc_uc (self , other) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_bool_char > for vector_unsigned_char { type Result = vector_unsigned_char ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_bool_char) -> Self :: Result { other . vec_add (self) } }}}

macro_rules! vec_add_uc_uc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_uc_uc in module {}", module_path!());
    };
}

mkfn!{
    vec_add_uc_uc_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vaddubm))] pub unsafe fn vec_add_uc_uc (a : vector_unsigned_char , b : vector_unsigned_char ,) -> vector_unsigned_char { simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_unsigned_char > for vector_unsigned_char { type Result = vector_unsigned_char ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_unsigned_char) -> Self :: Result { vec_add_uc_uc (self , other) } }}}

macro_rules! vec_add_bs_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_bs_ss in module {}", module_path!());
    };
}

mkfn!{
    vec_add_bs_ss_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduhm))] pub unsafe fn vec_add_bs_ss (a : vector_bool_short , b : vector_signed_short ,) -> vector_signed_short { let a : i16x8 = transmute (a) ; let a : vector_signed_short = simd_cast (a) ; simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_signed_short > for vector_bool_short { type Result = vector_signed_short ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_signed_short) -> Self :: Result { vec_add_bs_ss (self , other) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_bool_short > for vector_signed_short { type Result = vector_signed_short ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_bool_short) -> Self :: Result { other . vec_add (self) } }}}

macro_rules! vec_add_ss_ss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_ss_ss in module {}", module_path!());
    };
}

mkfn!{
    vec_add_ss_ss_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduhm))] pub unsafe fn vec_add_ss_ss (a : vector_signed_short , b : vector_signed_short ,) -> vector_signed_short { simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_signed_short > for vector_signed_short { type Result = vector_signed_short ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_signed_short) -> Self :: Result { vec_add_ss_ss (self , other) } }}}

macro_rules! vec_add_bs_us_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_bs_us in module {}", module_path!());
    };
}

mkfn!{
    vec_add_bs_us_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduhm))] pub unsafe fn vec_add_bs_us (a : vector_bool_short , b : vector_unsigned_short ,) -> vector_unsigned_short { let a : i16x8 = transmute (a) ; let a : vector_unsigned_short = simd_cast (a) ; simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_unsigned_short > for vector_bool_short { type Result = vector_unsigned_short ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_unsigned_short) -> Self :: Result { vec_add_bs_us (self , other) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_bool_short > for vector_unsigned_short { type Result = vector_unsigned_short ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_bool_short) -> Self :: Result { other . vec_add (self) } }}}

macro_rules! vec_add_us_us_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_us_us in module {}", module_path!());
    };
}

mkfn!{
    vec_add_us_us_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduhm))] pub unsafe fn vec_add_us_us (a : vector_unsigned_short , b : vector_unsigned_short ,) -> vector_unsigned_short { simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_unsigned_short > for vector_unsigned_short { type Result = vector_unsigned_short ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_unsigned_short) -> Self :: Result { vec_add_us_us (self , other) } }}}

macro_rules! vec_add_bi_si_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_bi_si in module {}", module_path!());
    };
}

mkfn!{
    vec_add_bi_si_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduwm))] pub unsafe fn vec_add_bi_si (a : vector_bool_int , b : vector_signed_int) -> vector_signed_int { let a : i32x4 = transmute (a) ; let a : vector_signed_int = simd_cast (a) ; simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_signed_int > for vector_bool_int { type Result = vector_signed_int ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_signed_int) -> Self :: Result { vec_add_bi_si (self , other) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_bool_int > for vector_signed_int { type Result = vector_signed_int ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_bool_int) -> Self :: Result { other . vec_add (self) } }}}

macro_rules! vec_add_si_si_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_si_si in module {}", module_path!());
    };
}

mkfn!{
    vec_add_si_si_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduwm))] pub unsafe fn vec_add_si_si (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int { simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_signed_int > for vector_signed_int { type Result = vector_signed_int ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_signed_int) -> Self :: Result { vec_add_si_si (self , other) } }}}

macro_rules! vec_add_bi_ui_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_bi_ui in module {}", module_path!());
    };
}

mkfn!{
    vec_add_bi_ui_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduwm))] pub unsafe fn vec_add_bi_ui (a : vector_bool_int , b : vector_unsigned_int) -> vector_unsigned_int { let a : i32x4 = transmute (a) ; let a : vector_unsigned_int = simd_cast (a) ; simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_unsigned_int > for vector_bool_int { type Result = vector_unsigned_int ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_unsigned_int) -> Self :: Result { vec_add_bi_ui (self , other) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_bool_int > for vector_unsigned_int { type Result = vector_unsigned_int ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_bool_int) -> Self :: Result { other . vec_add (self) } }}}

macro_rules! vec_add_ui_ui_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_ui_ui in module {}", module_path!());
    };
}

mkfn!{
    vec_add_ui_ui_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vadduwm))] pub unsafe fn vec_add_ui_ui (a : vector_unsigned_int , b : vector_unsigned_int ,) -> vector_unsigned_int { simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_unsigned_int > for vector_unsigned_int { type Result = vector_unsigned_int ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_unsigned_int) -> Self :: Result { vec_add_ui_ui (self , other) } }}}

macro_rules! vec_add_float_float_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_float_float in module {}", module_path!());
    };
}

mkfn!{
    vec_add_float_float_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (xvaddsp))] pub unsafe fn vec_add_float_float (a : vector_float , b : vector_float) -> vector_float { simd_add (a , b) }
}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdd < vector_float > for vector_float { type Result = vector_float ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_add (self , other : vector_float) -> Self :: Result { vec_add_float_float (self , other) } }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorAdde { unsafe fn vec_adde (self , b : Self , c : Self) -> Self ; }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdde for vector_unsigned_int { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_adde (self , b : Self , c : Self) -> Self { let mask : vector_unsigned_int = transmute (u32x4 :: new (1 , 1 , 1 , 1)) ; let carry = vec_and (c , mask) ; vec_add (vec_add (self , b) , carry) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorAdde for vector_signed_int { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_adde (self , b : Self , c : Self) -> Self { let mask : vector_signed_int = transmute (i32x4 :: new (1 , 1 , 1 , 1)) ; let carry = vec_and (c , mask) ; vec_add (vec_add (self , b) , carry) } }}}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMladd < Other > { type Result ; unsafe fn vec_mladd (self , b : Other , c : Other) -> Self :: Result ; }}}

macro_rules! mladd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mladd in module {}", module_path!());
    };
}

mkfn!{
    mladd_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmladduhm))] unsafe fn mladd (a : vector_signed_short , b : vector_signed_short , c : vector_signed_short ,) -> vector_signed_short { let a : i16x8 = transmute (a) ; let b : i16x8 = transmute (b) ; let c : i16x8 = transmute (c) ; transmute (simd_add (simd_mul (a , b) , c)) }
}
mkitem!{macro_rules ! vector_mladd { ($ a : ident , $ bc : ident , $ d : ident) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorMladd <$ bc > for $ a { type Result = $ d ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_mladd (self , b : $ bc , c : $ bc) -> Self :: Result { let a = transmute (self) ; let b = transmute (b) ; let c = transmute (c) ; transmute (mladd (a , b , c)) } } } ; }}
mkitem!{vector_mladd ! { vector_unsigned_short , vector_unsigned_short , vector_unsigned_short }}
mkitem!{vector_mladd ! { vector_unsigned_short , vector_signed_short , vector_signed_short }}
mkitem!{vector_mladd ! { vector_signed_short , vector_unsigned_short , vector_signed_short }}
mkitem!{vector_mladd ! { vector_signed_short , vector_signed_short , vector_signed_short }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorOr < Other > { type Result ; unsafe fn vec_or (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorOr vec_or] ~ (simd_or) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorXor < Other > { type Result ; unsafe fn vec_xor (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorXor vec_xor] ~ (simd_xor) }}
mkitem!{macro_rules ! vector_vnor { ($ fun : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "vsx")) , assert_instr (vnor))] # [cfg_attr (all (test , target_feature = "vsx") , assert_instr (xxlnor))] pub unsafe fn $ fun (a : t_t_l ! ($ ty) , b : t_t_l ! ($ ty)) -> t_t_l ! ($ ty) { let o = vec_splats (! 0 as $ ty) ; vec_xor (vec_or (a , b) , o) } } ; }}
mkitem!{vector_vnor ! { vec_vnorsb i8 }}
mkitem!{vector_vnor ! { vec_vnorsh i16 }}
mkitem!{vector_vnor ! { vec_vnorsw i32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorNor < Other > { type Result ; unsafe fn vec_nor (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorNor vec_nor] + 2b (vec_vnorsb , vec_vnorsh , vec_vnorsw) }}
mkitem!{macro_rules ! vector_vnand { ($ fun : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "vsx")) , assert_instr (vnand))] # [cfg_attr (all (test , target_feature = "vsx") , assert_instr (xxlnand))] pub unsafe fn $ fun (a : t_t_l ! ($ ty) , b : t_t_l ! ($ ty)) -> t_t_l ! ($ ty) { let o = vec_splats (! 0 as $ ty) ; vec_xor (vec_and (a , b) , o) } } ; }}
mkitem!{vector_vnand ! { vec_vnandsb i8 }}
mkitem!{vector_vnand ! { vec_vnandsh i16 }}
mkitem!{vector_vnand ! { vec_vnandsw i32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorNand < Other > { type Result ; unsafe fn vec_nand (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorNand vec_nand] + 2b (vec_vnandsb , vec_vnandsh , vec_vnandsw) }}

macro_rules! vec_vsel_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vsel in module {}", module_path!());
    };
}

mkfn!{
    vec_vsel_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , not (target_feature = "vsx")) , assert_instr (vsel))] # [cfg_attr (all (test , target_feature = "vsx") , assert_instr (xxsel))] pub unsafe fn vec_vsel (a : vector_signed_char , b : vector_signed_char , c : vector_signed_char ,) -> vector_signed_char { let a : i8x16 = transmute (a) ; let b : i8x16 = transmute (b) ; let c : i8x16 = transmute (c) ; let not_c = simd_xor (c , i8x16 :: splat (! 0)) ; transmute (simd_or (simd_and (a , not_c) , simd_and (b , c))) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSel < Mask > { unsafe fn vec_sel (self , b : Self , c : Mask) -> Self ; }}}
mkitem!{macro_rules ! vector_sel { ($ ty : ty , $ m : ty) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSel <$ m > for $ ty { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_sel (self , b : Self , c : $ m) -> Self { let a = transmute (self) ; let b = transmute (b) ; let c = transmute (c) ; transmute (vec_vsel (a , b , c)) } } } ; ($ ty : ident) => { vector_sel ! { $ ty , t_b ! { $ ty } } vector_sel ! { $ ty , t_u ! { $ ty } } vector_sel ! { t_u ! { $ ty } , t_b ! { $ ty } } vector_sel ! { t_u ! { $ ty } , t_u ! { $ ty } } vector_sel ! { t_b ! { $ ty } , t_b ! { $ ty } } vector_sel ! { t_b ! { $ ty } , t_u ! { $ ty } } } ; (- $ ty : ident) => { vector_sel ! { $ ty , t_b ! { $ ty } } vector_sel ! { $ ty , t_u ! { $ ty } } } ; }}
mkitem!{vector_sel ! { vector_signed_char }}
mkitem!{vector_sel ! { vector_signed_short }}
mkitem!{vector_sel ! { vector_signed_int }}
mkitem!{vector_sel ! { - vector_float }}

macro_rules! vec_ctf_i32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ctf_i32 in module {}", module_path!());
    };
}

mkfn!{
    vec_ctf_i32_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcfsx , IMM5 = 1))] unsafe fn vec_ctf_i32 < const IMM5 : i32 > (a : vector_signed_int) -> vector_float { static_assert_uimm_bits ! (IMM5 , 5) ; vcfsx (a , IMM5) }
}

macro_rules! vec_ctf_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ctf_u32 in module {}", module_path!());
    };
}

mkfn!{
    vec_ctf_u32_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vcfux , IMM5 = 1))] unsafe fn vec_ctf_u32 < const IMM5 : i32 > (a : vector_unsigned_int) -> vector_float { static_assert_uimm_bits ! (IMM5 , 5) ; vcfux (a , IMM5) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorCtf { unsafe fn vec_ctf < const IMM5 : i32 > (self) -> vector_float ; }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorCtf for vector_signed_int { unsafe fn vec_ctf < const IMM5 : i32 > (self) -> vector_float { vec_ctf_i32 :: < IMM5 > (self) } }}}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorCtf for vector_unsigned_int { unsafe fn vec_ctf < const IMM5 : i32 > (self) -> vector_float { vec_ctf_u32 :: < IMM5 > (self) } }}}

macro_rules! vec_vmrglb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmrglb in module {}", module_path!());
    };
}

mkfn!{
    vec_vmrglb_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , target_endian = "little") , assert_instr (vmrghb))] # [cfg_attr (all (test , target_endian = "big") , assert_instr (vmrglb))] unsafe fn vec_vmrglb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char { let mergel_perm = transmute (u8x16 :: new (0x08 , 0x18 , 0x09 , 0x19 , 0x0A , 0x1A , 0x0B , 0x1B , 0x0C , 0x1C , 0x0D , 0x1D , 0x0E , 0x1E , 0x0F , 0x1F ,)) ; vec_perm (a , b , mergel_perm) }
}

macro_rules! vec_vmrglh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmrglh in module {}", module_path!());
    };
}

mkfn!{
    vec_vmrglh_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , target_endian = "little") , assert_instr (vmrghh))] # [cfg_attr (all (test , target_endian = "big") , assert_instr (vmrglh))] unsafe fn vec_vmrglh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short { let mergel_perm = transmute (u8x16 :: new (0x08 , 0x09 , 0x18 , 0x19 , 0x0A , 0x0B , 0x1A , 0x1B , 0x0C , 0x0D , 0x1C , 0x1D , 0x0E , 0x0F , 0x1E , 0x1F ,)) ; vec_perm (a , b , mergel_perm) }
}

macro_rules! vec_vmrglw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmrglw in module {}", module_path!());
    };
}

mkfn!{
    vec_vmrglw_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , target_endian = "little" , not (target_feature = "vsx")) , assert_instr (vmrghw))] # [cfg_attr (all (test , target_endian = "little" , target_feature = "vsx") , assert_instr (xxmrghw))] # [cfg_attr (all (test , target_endian = "big" , not (target_feature = "vsx")) , assert_instr (vmrglw))] # [cfg_attr (all (test , target_endian = "big" , target_feature = "vsx") , assert_instr (xxmrglw))] unsafe fn vec_vmrglw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int { let mergel_perm = transmute (u8x16 :: new (0x08 , 0x09 , 0x0A , 0x0B , 0x18 , 0x19 , 0x1A , 0x1B , 0x0C , 0x0D , 0x0E , 0x0F , 0x1C , 0x1D , 0x1E , 0x1F ,)) ; vec_perm (a , b , mergel_perm) }
}

macro_rules! vec_vmrghb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmrghb in module {}", module_path!());
    };
}

mkfn!{
    vec_vmrghb_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , target_endian = "little") , assert_instr (vmrglb))] # [cfg_attr (all (test , target_endian = "big") , assert_instr (vmrghb))] unsafe fn vec_vmrghb (a : vector_signed_char , b : vector_signed_char) -> vector_signed_char { let mergel_perm = transmute (u8x16 :: new (0x00 , 0x10 , 0x01 , 0x11 , 0x02 , 0x12 , 0x03 , 0x13 , 0x04 , 0x14 , 0x05 , 0x15 , 0x06 , 0x16 , 0x07 , 0x17 ,)) ; vec_perm (a , b , mergel_perm) }
}

macro_rules! vec_vmrghh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmrghh in module {}", module_path!());
    };
}

mkfn!{
    vec_vmrghh_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , target_endian = "little") , assert_instr (vmrglh))] # [cfg_attr (all (test , target_endian = "big") , assert_instr (vmrghh))] unsafe fn vec_vmrghh (a : vector_signed_short , b : vector_signed_short) -> vector_signed_short { let mergel_perm = transmute (u8x16 :: new (0x00 , 0x01 , 0x10 , 0x11 , 0x02 , 0x03 , 0x12 , 0x13 , 0x04 , 0x05 , 0x14 , 0x15 , 0x06 , 0x07 , 0x16 , 0x17 ,)) ; vec_perm (a , b , mergel_perm) }
}

macro_rules! vec_vmrghw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vmrghw in module {}", module_path!());
    };
}

mkfn!{
    vec_vmrghw_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , target_endian = "little" , not (target_feature = "vsx")) , assert_instr (vmrglw))] # [cfg_attr (all (test , target_endian = "little" , target_feature = "vsx") , assert_instr (xxmrglw))] # [cfg_attr (all (test , target_endian = "big" , not (target_feature = "vsx")) , assert_instr (vmrghw))] # [cfg_attr (all (test , target_endian = "big" , target_feature = "vsx") , assert_instr (xxmrghw))] unsafe fn vec_vmrghw (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int { let mergel_perm = transmute (u8x16 :: new (0x00 , 0x01 , 0x02 , 0x03 , 0x10 , 0x11 , 0x12 , 0x13 , 0x04 , 0x05 , 0x06 , 0x07 , 0x14 , 0x15 , 0x16 , 0x17 ,)) ; vec_perm (a , b , mergel_perm) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMergeh < Other > { type Result ; unsafe fn vec_mergeh (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorMergeh vec_mergeh] + 2b (vec_vmrghb , vec_vmrghh , vec_vmrghw) }}
mkitem!{impl_vec_trait ! { [VectorMergeh vec_mergeh] + vec_vmrghw (vector_float , vector_float) -> vector_float }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorMergel < Other > { type Result ; unsafe fn vec_mergel (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorMergel vec_mergel] + 2b (vec_vmrglb , vec_vmrglh , vec_vmrglw) }}
mkitem!{impl_vec_trait ! { [VectorMergel vec_mergel] + vec_vmrglw (vector_float , vector_float) -> vector_float }}

macro_rules! vec_vpkuhum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkuhum in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkuhum_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkuhum))] unsafe fn vec_vpkuhum (a : vector_signed_short , b : vector_signed_short) -> vector_signed_char { let pack_perm = if cfg ! (target_endian = "little") { transmute (u8x16 :: new (0x00 , 0x02 , 0x04 , 0x06 , 0x08 , 0x0A , 0x0C , 0x0E , 0x10 , 0x12 , 0x14 , 0x16 , 0x18 , 0x1A , 0x1C , 0x1E ,)) } else { transmute (u8x16 :: new (0x01 , 0x03 , 0x05 , 0x07 , 0x09 , 0x0B , 0x0D , 0x0F , 0x11 , 0x13 , 0x15 , 0x17 , 0x19 , 0x1B , 0x1D , 0x1F ,)) } ; transmute (vec_perm (a , b , pack_perm)) }
}

macro_rules! vec_vpkuwum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkuwum in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkuwum_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkuwum))] unsafe fn vec_vpkuwum (a : vector_signed_int , b : vector_signed_int) -> vector_signed_short { let pack_perm = if cfg ! (target_endian = "little") { transmute (u8x16 :: new (0x00 , 0x01 , 0x04 , 0x05 , 0x08 , 0x09 , 0x0C , 0x0D , 0x10 , 0x11 , 0x14 , 0x15 , 0x18 , 0x19 , 0x1C , 0x1D ,)) } else { transmute (u8x16 :: new (0x02 , 0x03 , 0x06 , 0x07 , 0x0A , 0x0B , 0x0E , 0x0F , 0x12 , 0x13 , 0x16 , 0x17 , 0x1A , 0x1B , 0x1E , 0x1F ,)) } ; transmute (vec_perm (a , b , pack_perm)) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorPack < Other > { type Result ; unsafe fn vec_pack (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorPack vec_pack] + vec_vpkuhum (vector_signed_short , vector_signed_short) -> vector_signed_char }}
mkitem!{impl_vec_trait ! { [VectorPack vec_pack] + vec_vpkuhum (vector_unsigned_short , vector_unsigned_short) -> vector_unsigned_char }}
mkitem!{impl_vec_trait ! { [VectorPack vec_pack] + vec_vpkuhum (vector_bool_short , vector_bool_short) -> vector_bool_char }}
mkitem!{impl_vec_trait ! { [VectorPack vec_pack] + vec_vpkuwum (vector_signed_int , vector_signed_int) -> vector_signed_short }}
mkitem!{impl_vec_trait ! { [VectorPack vec_pack] + vec_vpkuwum (vector_unsigned_int , vector_unsigned_int) -> vector_unsigned_short }}
mkitem!{impl_vec_trait ! { [VectorPack vec_pack] + vec_vpkuwum (vector_bool_int , vector_bool_int) -> vector_bool_short }}

macro_rules! vec_vpkshss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkshss in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkshss_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkshss))] unsafe fn vec_vpkshss (a : vector_signed_short , b : vector_signed_short) -> vector_signed_char { if cfg ! (target_endian = "little") { vpkshss (b , a) } else { vpkshss (a , b) } }
}

macro_rules! vec_vpkshus_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkshus in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkshus_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkshus))] unsafe fn vec_vpkshus (a : vector_signed_short , b : vector_signed_short) -> vector_unsigned_char { if cfg ! (target_endian = "little") { vpkshus (b , a) } else { vpkshus (a , b) } }
}

macro_rules! vec_vpkuhus_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkuhus in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkuhus_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkuhus))] unsafe fn vec_vpkuhus (a : vector_unsigned_short , b : vector_unsigned_short ,) -> vector_unsigned_char { if cfg ! (target_endian = "little") { vpkuhus (b , a) } else { vpkuhus (a , b) } }
}

macro_rules! vec_vpkswss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkswss in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkswss_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkswss))] unsafe fn vec_vpkswss (a : vector_signed_int , b : vector_signed_int) -> vector_signed_short { if cfg ! (target_endian = "little") { vpkswss (b , a) } else { vpkswss (a , b) } }
}

macro_rules! vec_vpkswus_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkswus in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkswus_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkswus))] unsafe fn vec_vpkswus (a : vector_signed_int , b : vector_signed_int) -> vector_unsigned_short { if cfg ! (target_endian = "little") { vpkswus (b , a) } else { vpkswus (a , b) } }
}

macro_rules! vec_vpkuwus_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_vpkuwus in module {}", module_path!());
    };
}

mkfn!{
    vec_vpkuwus_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vpkuwus))] unsafe fn vec_vpkuwus (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_short { if cfg ! (target_endian = "little") { vpkuwus (b , a) } else { vpkuwus (a , b) } }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorPacks < Other > { type Result ; unsafe fn vec_packs (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorPacks vec_packs] vec_vpkshss (vector_signed_short , vector_signed_short) -> vector_signed_char }}
mkitem!{impl_vec_trait ! { [VectorPacks vec_packs] vec_vpkuhus (vector_unsigned_short , vector_unsigned_short) -> vector_unsigned_char }}
mkitem!{impl_vec_trait ! { [VectorPacks vec_packs] vec_vpkswss (vector_signed_int , vector_signed_int) -> vector_signed_short }}
mkitem!{impl_vec_trait ! { [VectorPacks vec_packs] vec_vpkuwus (vector_unsigned_int , vector_unsigned_int) -> vector_unsigned_short }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorPacksu < Other > { type Result ; unsafe fn vec_packsu (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorPacksu vec_packsu] vec_vpkshus (vector_signed_short , vector_signed_short) -> vector_unsigned_char }}
mkitem!{impl_vec_trait ! { [VectorPacksu vec_packsu] vec_vpkuhus (vector_unsigned_short , vector_unsigned_short) -> vector_unsigned_char }}
mkitem!{impl_vec_trait ! { [VectorPacksu vec_packsu] vec_vpkswus (vector_signed_int , vector_signed_int) -> vector_unsigned_short }}
mkitem!{impl_vec_trait ! { [VectorPacksu vec_packsu] vec_vpkuwus (vector_unsigned_int , vector_unsigned_int) -> vector_unsigned_short }}
mkitem!{macro_rules ! impl_vec_unpack { ($ fun : ident ($ a : ident) -> $ r : ident [$ little : ident , $ big : ident]) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (all (test , target_endian = "little") , assert_instr ($ little))] # [cfg_attr (all (test , target_endian = "big") , assert_instr ($ big))] unsafe fn $ fun (a : $ a) -> $ r { if cfg ! (target_endian = "little") { $ little (a) } else { $ big (a) } } } ; }}
mkitem!{impl_vec_unpack ! { vec_vupkhsb (vector_signed_char) -> vector_signed_short [vupklsb , vupkhsb] }}
mkitem!{impl_vec_unpack ! { vec_vupklsb (vector_signed_char) -> vector_signed_short [vupkhsb , vupklsb] }}
mkitem!{impl_vec_unpack ! { vec_vupkhsh (vector_signed_short) -> vector_signed_int [vupklsh , vupkhsh] }}
mkitem!{impl_vec_unpack ! { vec_vupklsh (vector_signed_short) -> vector_signed_int [vupkhsh , vupklsh] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorUnpackh { type Result ; unsafe fn vec_unpackh (self) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorUnpackh vec_unpackh] vec_vupkhsb (vector_signed_char) -> vector_signed_short }}
mkitem!{impl_vec_trait ! { [VectorUnpackh vec_unpackh] + vec_vupkhsb (vector_bool_char) -> vector_bool_short }}
mkitem!{impl_vec_trait ! { [VectorUnpackh vec_unpackh] vec_vupkhsh (vector_signed_short) -> vector_signed_int }}
mkitem!{impl_vec_trait ! { [VectorUnpackh vec_unpackh] + vec_vupkhsh (vector_bool_short) -> vector_bool_int }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorUnpackl { type Result ; unsafe fn vec_unpackl (self) -> Self :: Result ; }}}
mkitem!{impl_vec_trait ! { [VectorUnpackl vec_unpackl] vec_vupklsb (vector_signed_char) -> vector_signed_short }}
mkitem!{impl_vec_trait ! { [VectorUnpackl vec_unpackl] + vec_vupklsb (vector_bool_char) -> vector_bool_short }}
mkitem!{impl_vec_trait ! { [VectorUnpackl vec_unpackl] vec_vupklsh (vector_signed_short) -> vector_signed_int }}
mkitem!{impl_vec_trait ! { [VectorUnpackl vec_unpackl] + vec_vupklsh (vector_bool_short) -> vector_bool_int }}
mkitem!{macro_rules ! impl_vec_shift { ([$ Trait : ident $ m : ident] ($ b : ident , $ h : ident , $ w : ident)) => { impl_vec_trait ! { [$ Trait $ m] + $ b (vector_unsigned_char , vector_unsigned_char) -> vector_unsigned_char } impl_vec_trait ! { [$ Trait $ m] + $ b (vector_signed_char , vector_unsigned_char) -> vector_signed_char } impl_vec_trait ! { [$ Trait $ m] + $ h (vector_unsigned_short , vector_unsigned_short) -> vector_unsigned_short } impl_vec_trait ! { [$ Trait $ m] + $ h (vector_signed_short , vector_unsigned_short) -> vector_signed_short } impl_vec_trait ! { [$ Trait $ m] + $ w (vector_unsigned_int , vector_unsigned_int) -> vector_unsigned_int } impl_vec_trait ! { [$ Trait $ m] + $ w (vector_signed_int , vector_unsigned_int) -> vector_signed_int } } ; }}
mkitem!{macro_rules ! impl_shift { ($ fun : ident $ intr : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ($ fun))] unsafe fn $ fun (a : t_t_l ! ($ ty) , b : t_t_l ! ($ ty)) -> t_t_l ! ($ ty) { let a = transmute (a) ; let b = simd_rem (transmute (b) , < t_t_s ! ($ ty) >:: splat (mem :: size_of ::<$ ty > () as $ ty * $ ty :: BITS as $ ty) ,) ; transmute ($ intr (a , b)) } } ; }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSl < Other > { type Result ; unsafe fn vec_sl (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_shift ! { vslb simd_shl u8 }}
mkitem!{impl_shift ! { vslh simd_shl u16 }}
mkitem!{impl_shift ! { vslw simd_shl u32 }}
mkitem!{impl_vec_shift ! { [VectorSl vec_sl] (vslb , vslh , vslw) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSr < Other > { type Result ; unsafe fn vec_sr (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_shift ! { vsrb simd_shr u8 }}
mkitem!{impl_shift ! { vsrh simd_shr u16 }}
mkitem!{impl_shift ! { vsrw simd_shr u32 }}
mkitem!{impl_vec_shift ! { [VectorSr vec_sr] (vsrb , vsrh , vsrw) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSra < Other > { type Result ; unsafe fn vec_sra (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_shift ! { [VectorSra vec_sra] (vsrab , vsrah , vsraw) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSld { unsafe fn vec_sld < const UIMM4 : i32 > (self , b : Self) -> Self ; unsafe fn vec_sldw < const UIMM2 : i32 > (self , b : Self) -> Self ; }}}

macro_rules! vsldoi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vsldoi in module {}", module_path!());
    };
}

mkfn!{
    vsldoi_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vsldoi , UIMM4 = 1))] unsafe fn vsldoi < const UIMM4 : i32 > (a : vector_unsigned_char , b : vector_unsigned_char ,) -> vector_unsigned_char { static_assert_uimm_bits ! (UIMM4 , 4) ; let d = UIMM4 as u8 ; if cfg ! (target_endian = "little") { let perm = u8x16 :: new (16 - d , 17 - d , 18 - d , 19 - d , 20 - d , 21 - d , 22 - d , 23 - d , 24 - d , 25 - d , 26 - d , 27 - d , 28 - d , 29 - d , 30 - d , 31 - d ,) ; vec_perm (b , a , transmute (perm)) } else { let perm = u8x16 :: new (d , d + 1 , d + 2 , d + 3 , d + 4 , d + 5 , d + 6 , d + 7 , d + 8 , d + 9 , d + 10 , d + 11 , d + 12 , d + 13 , d + 14 , d + 15 ,) ; vec_perm (a , b , transmute (perm)) } }
}

macro_rules! xxsldwi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function xxsldwi in module {}", module_path!());
    };
}

mkfn!{
    xxsldwi_introspect!();
    # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (xxsldwi , UIMM2 = 1))] unsafe fn xxsldwi < const UIMM2 : i32 > (a : vector_unsigned_char , b : vector_unsigned_char ,) -> vector_unsigned_char { static_assert_uimm_bits ! (UIMM2 , 2) ; let d = (UIMM2 << 2) as u8 ; if cfg ! (target_endian = "little") { let perm = u8x16 :: new (16 - d , 17 - d , 18 - d , 19 - d , 20 - d , 21 - d , 22 - d , 23 - d , 24 - d , 25 - d , 26 - d , 27 - d , 28 - d , 29 - d , 30 - d , 31 - d ,) ; vec_perm (b , a , transmute (perm)) } else { let perm = u8x16 :: new (d , d + 1 , d + 2 , d + 3 , d + 4 , d + 5 , d + 6 , d + 7 , d + 8 , d + 9 , d + 10 , d + 11 , d + 12 , d + 13 , d + 14 , d + 15 ,) ; vec_perm (a , b , transmute (perm)) } }
}
mkitem!{macro_rules ! impl_vec_sld { ($ ($ ty : ident) ,+) => { $ (# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorSld for $ ty { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_sld < const UIMM4 : i32 > (self , b : Self) -> Self { transmute (vsldoi ::< UIMM4 > (transmute (self) , transmute (b))) } # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_sldw < const UIMM2 : i32 > (self , b : Self) -> Self { transmute (xxsldwi ::< UIMM2 > (transmute (self) , transmute (b))) } }) + } ; }}
mkitem!{impl_vec_sld ! { vector_bool_char , vector_signed_char , vector_unsigned_char }}
mkitem!{impl_vec_sld ! { vector_bool_short , vector_signed_short , vector_unsigned_short }}
mkitem!{impl_vec_sld ! { vector_bool_int , vector_signed_int , vector_unsigned_int }}
mkitem!{impl_vec_sld ! { vector_float }}
mkitem!{macro_rules ! impl_vec_shift_long { ([$ Trait : ident $ m : ident] ($ f : ident)) => { impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_char , vector_unsigned_char) -> vector_unsigned_char } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_char , vector_unsigned_char) -> vector_signed_char } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_short , vector_unsigned_char) -> vector_unsigned_short } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_short , vector_unsigned_char) -> vector_signed_short } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_int , vector_unsigned_char) -> vector_unsigned_int } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_int , vector_unsigned_char) -> vector_signed_int } } ; }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSll < Other > { type Result ; unsafe fn vec_sll (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_shift_long ! { [VectorSll vec_sll] (vsl) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSrl < Other > { type Result ; unsafe fn vec_srl (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_shift_long ! { [VectorSrl vec_srl] (vsr) }}
mkitem!{macro_rules ! impl_vec_shift_octect { ([$ Trait : ident $ m : ident] ($ f : ident)) => { impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_char , vector_signed_char) -> vector_unsigned_char } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_char , vector_signed_char) -> vector_signed_char } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_short , vector_signed_char) -> vector_unsigned_short } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_short , vector_signed_char) -> vector_signed_short } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_int , vector_signed_char) -> vector_unsigned_int } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_int , vector_signed_char) -> vector_signed_int } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_float , vector_signed_char) -> vector_float } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_char , vector_unsigned_char) -> vector_unsigned_char } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_char , vector_unsigned_char) -> vector_signed_char } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_short , vector_unsigned_char) -> vector_unsigned_short } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_short , vector_unsigned_char) -> vector_signed_short } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_unsigned_int , vector_unsigned_char) -> vector_unsigned_int } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_signed_int , vector_unsigned_char) -> vector_signed_int } impl_vec_trait ! { [$ Trait $ m] + $ f (vector_float , vector_unsigned_char) -> vector_float } } ; }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSlo < Other > { type Result ; unsafe fn vec_slo (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_shift_octect ! { [VectorSlo vec_slo] (vslo) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorSro < Other > { type Result ; unsafe fn vec_sro (self , b : Other) -> Self :: Result ; }}}
mkitem!{impl_vec_shift_octect ! { [VectorSro vec_sro] (vsro) }}
mkitem!{test_impl ! { vec_vcntlzb (a : vector_signed_char) -> vector_signed_char [simd_ctlz , vclzb] }}
mkitem!{test_impl ! { vec_vcntlzh (a : vector_signed_short) -> vector_signed_short [simd_ctlz , vclzh] }}
mkitem!{test_impl ! { vec_vcntlzw (a : vector_signed_int) -> vector_signed_int [simd_ctlz , vclzw] }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorCntlz { unsafe fn vec_cntlz (self) -> Self ; }}}
mkitem!{macro_rules ! impl_vec_cntlz { ($ fun : ident ($ a : ty)) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorCntlz for $ a { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_cntlz (self) -> Self { transmute ($ fun (transmute (self))) } } } ; }}
mkitem!{impl_vec_cntlz ! { vec_vcntlzb (vector_signed_char) }}
mkitem!{impl_vec_cntlz ! { vec_vcntlzb (vector_unsigned_char) }}
mkitem!{impl_vec_cntlz ! { vec_vcntlzh (vector_signed_short) }}
mkitem!{impl_vec_cntlz ! { vec_vcntlzh (vector_unsigned_short) }}
mkitem!{impl_vec_cntlz ! { vec_vcntlzw (vector_signed_int) }}
mkitem!{impl_vec_cntlz ! { vec_vcntlzw (vector_unsigned_int) }}
mkitem!{macro_rules ! impl_vrl { ($ fun : ident $ ty : ident) => { # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ($ fun))] unsafe fn $ fun (a : t_t_l ! ($ ty) , b : t_t_l ! ($ ty)) -> t_t_l ! ($ ty) { simd_funnel_shl (a , a , b) } } ; }}
mkitem!{impl_vrl ! { vrlb u8 }}
mkitem!{impl_vrl ! { vrlh u16 }}
mkitem!{impl_vrl ! { vrlw u32 }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorRl { type Shift ; unsafe fn vec_rl (self , b : Self :: Shift) -> Self ; }}}
mkitem!{macro_rules ! impl_vec_rl { ($ fun : ident ($ a : ident)) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorRl for $ a { type Shift = t_u ! ($ a) ; # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_rl (self , b : Self :: Shift) -> Self { transmute ($ fun (transmute (self) , b)) } } } ; }}
mkitem!{impl_vec_rl ! { vrlb (vector_signed_char) }}
mkitem!{impl_vec_rl ! { vrlh (vector_signed_short) }}
mkitem!{impl_vec_rl ! { vrlw (vector_signed_int) }}
mkitem!{impl_vec_rl ! { vrlb (vector_unsigned_char) }}
mkitem!{impl_vec_rl ! { vrlh (vector_unsigned_short) }}
mkitem!{impl_vec_rl ! { vrlw (vector_unsigned_int) }}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorRound { unsafe fn vec_round (self) -> Self ; }}}
mkitem!{test_impl ! { vec_vrfin (a : vector_float) -> vector_float [vrfin , xvrspic] }}
mkitem!{mkimpl!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorRound for vector_float { # [inline] # [target_feature (enable = "altivec")] unsafe fn vec_round (self) -> Self { vec_vrfin (self) } }}} 
            }}

macro_rules! vec_insert_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_insert in module {}", module_path!());
    };
}

mkfn!{
    vec_insert_introspect!();
    # [doc = " Vector Insert"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Returns a copy of vector b with element c replaced by the value of a."] # [doc = ""] # [doc = " ## Result value"] # [doc = " r contains a copy of vector b with element c replaced by the value of a."] # [doc = " This function uses modular arithmetic on c to determine the element number."] # [doc = " For example, if c is out of range, the compiler uses c modulo the number of"] # [doc = " elements in the vector to determine the element position."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_insert < T , const IDX : u32 > (a : T , b : < T as sealed :: VectorInsert > :: Scalar) -> T where T : sealed :: VectorInsert , { a . vec_insert :: < IDX > (b) }
}

macro_rules! vec_extract_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_extract in module {}", module_path!());
    };
}

mkfn!{
    vec_extract_introspect!();
    # [doc = " Vector Extract"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Returns the value of the bth element of vector a."] # [doc = ""] # [doc = " ## Result value"] # [doc = " The value of each element of r is the element of a at position b modulo the number of"] # [doc = " elements of a."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_extract < T , const IDX : u32 > (a : T) -> < T as sealed :: VectorExtract > :: Scalar where T : sealed :: VectorExtract , { a . vec_extract :: < IDX > () }
}

macro_rules! vec_mergel_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mergel in module {}", module_path!());
    };
}

mkfn!{
    vec_mergel_introspect!();
    # [doc = " Vector Merge Low"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mergel < T , U > (a : T , b : U) -> < T as sealed :: VectorMergel < U > > :: Result where T : sealed :: VectorMergel < U > , { a . vec_mergel (b) }
}

macro_rules! vec_mergeh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mergeh in module {}", module_path!());
    };
}

mkfn!{
    vec_mergeh_introspect!();
    # [doc = " Vector Merge High"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mergeh < T , U > (a : T , b : U) -> < T as sealed :: VectorMergeh < U > > :: Result where T : sealed :: VectorMergeh < U > , { a . vec_mergeh (b) }
}

macro_rules! vec_pack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_pack in module {}", module_path!());
    };
}

mkfn!{
    vec_pack_introspect!();
    # [doc = " Vector Pack"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_pack < T , U > (a : T , b : U) -> < T as sealed :: VectorPack < U > > :: Result where T : sealed :: VectorPack < U > , { a . vec_pack (b) }
}

macro_rules! vec_packs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_packs in module {}", module_path!());
    };
}

mkfn!{
    vec_packs_introspect!();
    # [doc = " Vector Pack Saturated"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_packs < T , U > (a : T , b : U) -> < T as sealed :: VectorPacks < U > > :: Result where T : sealed :: VectorPacks < U > , { a . vec_packs (b) }
}

macro_rules! vec_packsu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_packsu in module {}", module_path!());
    };
}

mkfn!{
    vec_packsu_introspect!();
    # [doc = " Vector Pack Saturated Unsigned"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_packsu < T , U > (a : T , b : U) -> < T as sealed :: VectorPacksu < U > > :: Result where T : sealed :: VectorPacksu < U > , { a . vec_packsu (b) }
}

macro_rules! vec_unpackh_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_unpackh in module {}", module_path!());
    };
}

mkfn!{
    vec_unpackh_introspect!();
    # [doc = " Vector Unpack High"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_unpackh < T > (a : T) -> < T as sealed :: VectorUnpackh > :: Result where T : sealed :: VectorUnpackh , { a . vec_unpackh () }
}

macro_rules! vec_unpackl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_unpackl in module {}", module_path!());
    };
}

mkfn!{
    vec_unpackl_introspect!();
    # [doc = " Vector Unpack Low"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_unpackl < T > (a : T) -> < T as sealed :: VectorUnpackl > :: Result where T : sealed :: VectorUnpackl , { a . vec_unpackl () }
}

macro_rules! vec_sl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sl in module {}", module_path!());
    };
}

mkfn!{
    vec_sl_introspect!();
    # [doc = " Vector Shift Left"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sl < T , U > (a : T , b : U) -> < T as sealed :: VectorSl < U > > :: Result where T : sealed :: VectorSl < U > , { a . vec_sl (b) }
}

macro_rules! vec_sr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sr in module {}", module_path!());
    };
}

mkfn!{
    vec_sr_introspect!();
    # [doc = " Vector Shift Right"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sr < T , U > (a : T , b : U) -> < T as sealed :: VectorSr < U > > :: Result where T : sealed :: VectorSr < U > , { a . vec_sr (b) }
}

macro_rules! vec_sra_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sra in module {}", module_path!());
    };
}

mkfn!{
    vec_sra_introspect!();
    # [doc = " Vector Shift Right Algebraic"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sra < T , U > (a : T , b : U) -> < T as sealed :: VectorSra < U > > :: Result where T : sealed :: VectorSra < U > , { a . vec_sra (b) }
}

macro_rules! vec_sld_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sld in module {}", module_path!());
    };
}

mkfn!{
    vec_sld_introspect!();
    # [doc = " Vector Shift Left Double"] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = ""] # [doc = " This intrinsic is not endian-neutral, so uses of vec_sld in"] # [doc = " big-endian code must be rewritten for little-endian targets."] # [doc = ""] # [doc = " Historically, vec_sld could be used to shift by amounts not a multiple of the element size"] # [doc = " for most types, in which case the purpose of the shift is difficult to determine and difficult"] # [doc = " to automatically rewrite efficiently for little endian."] # [doc = ""] # [doc = " So the concatenation of a and b is done in big-endian fashion (left to right), and the shift is"] # [doc = " always to the left. This will generally produce surprising results for little-endian targets."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sld < T , const UIMM4 : i32 > (a : T , b : T) -> T where T : sealed :: VectorSld , { a . vec_sld :: < UIMM4 > (b) }
}

macro_rules! vec_sldw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sldw in module {}", module_path!());
    };
}

mkfn!{
    vec_sldw_introspect!();
    # [doc = " Vector Shift Left Double by Words"] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = ""] # [doc = " This intrinsic is not endian-neutral, so uses of vec_sldw in"] # [doc = " big-endian code must be rewritten for little-endian targets."] # [doc = ""] # [doc = " The concatenation of a and b is done in big-endian fashion (left to right), and the shift is"] # [doc = " always to the left. This will generally produce surprising results for little- endian targets."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sldw < T , const UIMM2 : i32 > (a : T , b : T) -> T where T : sealed :: VectorSld , { a . vec_sldw :: < UIMM2 > (b) }
}

macro_rules! vec_sll_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sll in module {}", module_path!());
    };
}

mkfn!{
    vec_sll_introspect!();
    # [doc = " Vector Shift Left Long"] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = " This intrinsic is not endian-neutral, so uses of vec_sll in big-endian"] # [doc = " code must be rewritten for little-endian targets."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sll < T , U > (a : T , b : U) -> < T as sealed :: VectorSll < U > > :: Result where T : sealed :: VectorSll < U > , { a . vec_sll (b) }
}

macro_rules! vec_srl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_srl in module {}", module_path!());
    };
}

mkfn!{
    vec_srl_introspect!();
    # [doc = " Vector Shift Right Long"] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = " This intrinsic is not endian-neutral, so uses of vec_srl in big-endian"] # [doc = " code must be rewritten for little-endian targets."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_srl < T , U > (a : T , b : U) -> < T as sealed :: VectorSrl < U > > :: Result where T : sealed :: VectorSrl < U > , { a . vec_srl (b) }
}

macro_rules! vec_slo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_slo in module {}", module_path!());
    };
}

mkfn!{
    vec_slo_introspect!();
    # [doc = " Vector Shift Left by Octets"] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = " This intrinsic is not endian-neutral, so uses of vec_slo in big-endian code must be rewritten"] # [doc = " for little-endian targets. The shift count is in element 15 of b for big-endian, but in element"] # [doc = " 0 of b for little-endian."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_slo < T , U > (a : T , b : U) -> < T as sealed :: VectorSlo < U > > :: Result where T : sealed :: VectorSlo < U > , { a . vec_slo (b) }
}

macro_rules! vec_sro_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sro in module {}", module_path!());
    };
}

mkfn!{
    vec_sro_introspect!();
    # [doc = " Vector Shift Right by Octets"] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = " This intrinsic is not endian-neutral, so uses of vec_sro in big-endian code must be rewritten"] # [doc = " for little-endian targets. The shift count is in element 15 of b for big-endian, but in element"] # [doc = " 0 of b for little-endian."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sro < T , U > (a : T , b : U) -> < T as sealed :: VectorSro < U > > :: Result where T : sealed :: VectorSro < U > , { a . vec_sro (b) }
}

macro_rules! vec_slv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_slv in module {}", module_path!());
    };
}

mkfn!{
    vec_slv_introspect!();
    # [doc = " Vector Shift Left Variable"] # [doc = ""] # [doc = " ## Result value"] # [doc = " Let v be a 17-byte vector formed from a in bytes `[0:15]` and a zero byte in element 16."] # [doc = " Then each byte element i of r is determined as follows. The start bit sb is"] # [doc = " obtained from bits 5:7 of byte element i of b. Then the contents of bits sb:sb+7 of the"] # [doc = " halfword in byte elements i:i+1 of v are placed into byte element i of r."] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = " All bit and byte element numbers are specified in big-endian order. This intrinsic is not"] # [doc = " endian-neutral."] # [inline] # [target_feature (enable = "power9-altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_slv (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char { vslv (a , b) }
}

macro_rules! vec_srv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_srv in module {}", module_path!());
    };
}

mkfn!{
    vec_srv_introspect!();
    # [doc = " Vector Shift Right Variable"] # [doc = ""] # [doc = " ## Result value"] # [doc = " Let v be a 17-byte vector formed from a zero byte in element 0 and the elements of"] # [doc = " a in bytes `[1:16]`. Then each byte element i of r is determined as follows. The start bit sb is"] # [doc = " obtained from bits 5:7 of byte element i of b. Then the contents of bits (8 – sb):(15 – sb) of"] # [doc = " the halfword in byte elements i:i+1 of v are placed into byte element i of r."] # [doc = ""] # [doc = " ## Endian considerations"] # [doc = " All bit and byte element numbers are specified in big-endian order. This intrinsic is not"] # [doc = " endian-neutral."] # [inline] # [target_feature (enable = "power9-altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_srv (a : vector_unsigned_char , b : vector_unsigned_char) -> vector_unsigned_char { vsrv (a , b) }
}

macro_rules! vec_ld_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ld in module {}", module_path!());
    };
}

mkfn!{
    vec_ld_introspect!();
    # [doc = " Vector Load Indexed."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_ld < T > (off : isize , p : T) -> < T as sealed :: VectorLd > :: Result where T : sealed :: VectorLd , { p . vec_ld (off) }
}

macro_rules! vec_ldl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ldl in module {}", module_path!());
    };
}

mkfn!{
    vec_ldl_introspect!();
    # [doc = " Vector Load Indexed Least Recently Used."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_ldl < T > (off : isize , p : T) -> < T as sealed :: VectorLd > :: Result where T : sealed :: VectorLd , { p . vec_ldl (off) }
}

macro_rules! vec_lde_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_lde in module {}", module_path!());
    };
}

mkfn!{
    vec_lde_introspect!();
    # [doc = " Vector Load Element Indexed."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_lde < T > (off : isize , p : T) -> < T as sealed :: VectorLde > :: Result where T : sealed :: VectorLde , { p . vec_lde (off) }
}

macro_rules! vec_st_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_st in module {}", module_path!());
    };
}

mkfn!{
    vec_st_introspect!();
    # [doc = " Vector Store Indexed"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Stores a 16-byte vector into memory at the address specified by a displacement and a"] # [doc = " pointer, ignoring the four low-order bits of the calculated address."] # [doc = ""] # [doc = " ## Operation"] # [doc = " A memory address is obtained by adding b and c, and masking off the four low-order"] # [doc = " bits of the result. The 16-byte vector in a is stored to the resultant memory address."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_st < T > (a : T , off : isize , c : < T as sealed :: VectorSt > :: Target) where T : sealed :: VectorSt , { a . vec_st (off , c) }
}

macro_rules! vec_stl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_stl in module {}", module_path!());
    };
}

mkfn!{
    vec_stl_introspect!();
    # [doc = " Vector Store Indexed Least Recently Used"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Stores a 16-byte vector into memory at the address specified by a displacement and"] # [doc = " a pointer, ignoring the four low-order bits of the calculated address, and marking the cache"] # [doc = " line containing the address as least frequently used."] # [doc = ""] # [doc = " ## Operation"] # [doc = " A memory address is obtained by adding b and c, and masking off the four"] # [doc = " low-order bits of the result. The 16-byte vector in a is stored to the resultant memory"] # [doc = " address, and the containing cache line is marked as least frequently used."] # [doc = ""] # [doc = " ## Notes"] # [doc = " This intrinsic can be used to indicate the last access to a portion of memory, as a hint to the"] # [doc = " data cache controller that the associated cache line can be replaced without performance loss."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_stl < T > (a : T , off : isize , c : < T as sealed :: VectorSt > :: Target) where T : sealed :: VectorSt , { a . vec_stl (off , c) }
}

macro_rules! vec_ste_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ste in module {}", module_path!());
    };
}

mkfn!{
    vec_ste_introspect!();
    # [doc = " Vector Store Element Indexed"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Stores a single element from a 16-byte vector into memory at the address specified by"] # [doc = " a displacement and a pointer, aligned to the element size."] # [doc = ""] # [doc = " ## Operation"] # [doc = " The integer value b is added to the pointer value c. The resulting address is"] # [doc = " rounded down to the nearest address that is a multiple of es, where es is 1 for char pointers,"] # [doc = " 2 for short pointers, and 4 for float or int pointers. An element offset eo is calculated by"] # [doc = " taking the resultant address modulo 16. The vector element of a at offset eo is stored to the"] # [doc = " resultant address."] # [doc = ""] # [doc = " ## Notes"] # [doc = " Be careful to note that the address (b+c) is aligned to an element boundary. Do not attempt"] # [doc = " to store unaligned data with this intrinsic."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_ste < T > (a : T , off : isize , c : < T as sealed :: VectorSte > :: Target) where T : sealed :: VectorSte , { a . vec_ste (off , c) }
}

macro_rules! vec_xl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_xl in module {}", module_path!());
    };
}

mkfn!{
    vec_xl_introspect!();
    # [doc = " VSX Unaligned Load"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_xl < T > (off : isize , p : T) -> < T as sealed :: VectorXl > :: Result where T : sealed :: VectorXl , { p . vec_xl (off) }
}

macro_rules! vec_xst_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_xst in module {}", module_path!());
    };
}

mkfn!{
    vec_xst_introspect!();
    # [doc = " VSX Unaligned Store"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_xst < T > (v : T , off : isize , p : < T as sealed :: VectorXst > :: Out) where T : sealed :: VectorXst , { v . vec_xst (off , p) }
}

macro_rules! vec_loge_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_loge in module {}", module_path!());
    };
}

mkfn!{
    vec_loge_introspect!();
    # [doc = " Vector Base-2 Logarithm Estimate"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vlogefp))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_loge (a : vector_float) -> vector_float { vlogefp (a) }
}

macro_rules! vec_floor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_floor in module {}", module_path!());
    };
}

mkfn!{
    vec_floor_introspect!();
    # [doc = " Vector floor."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_floor (a : vector_float) -> vector_float { sealed :: vec_floor (a) }
}

macro_rules! vec_expte_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_expte in module {}", module_path!());
    };
}

mkfn!{
    vec_expte_introspect!();
    # [doc = " Vector expte."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_expte (a : vector_float) -> vector_float { sealed :: vec_vexptefp (a) }
}

macro_rules! vec_cmplt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cmplt in module {}", module_path!());
    };
}

mkfn!{
    vec_cmplt_introspect!();
    # [doc = " Vector cmplt."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cmplt < T , U > (a : U , b : T) -> < T as sealed :: VectorCmpGt < U > > :: Result where T : sealed :: VectorCmpGt < U > , { vec_cmpgt (b , a) }
}

macro_rules! vec_cmple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cmple in module {}", module_path!());
    };
}

mkfn!{
    vec_cmple_introspect!();
    # [doc = " Vector cmple."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cmple (a : vector_float , b : vector_float) -> vector_bool_int { vec_cmpge (b , a) }
}

macro_rules! vec_cmpgt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cmpgt in module {}", module_path!());
    };
}

mkfn!{
    vec_cmpgt_introspect!();
    # [doc = " Vector cmpgt."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cmpgt < T , U > (a : T , b : U) -> < T as sealed :: VectorCmpGt < U > > :: Result where T : sealed :: VectorCmpGt < U > , { a . vec_cmpgt (b) }
}

macro_rules! vec_cmpge_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cmpge in module {}", module_path!());
    };
}

mkfn!{
    vec_cmpge_introspect!();
    # [doc = " Vector cmpge."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cmpge (a : vector_float , b : vector_float) -> vector_bool_int { sealed :: vec_vcmpgefp (a , b) }
}

macro_rules! vec_cmpeq_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cmpeq in module {}", module_path!());
    };
}

mkfn!{
    vec_cmpeq_introspect!();
    # [doc = " Vector cmpeq."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cmpeq < T , U > (a : T , b : U) -> < T as sealed :: VectorCmpEq < U > > :: Result where T : sealed :: VectorCmpEq < U > , { a . vec_cmpeq (b) }
}

macro_rules! vec_cmpne_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cmpne in module {}", module_path!());
    };
}

mkfn!{
    vec_cmpne_introspect!();
    # [doc = " Vector Compare Not Equal"] # [doc = ""] # [doc = " ## Result value"] # [doc = " For each element of r, the value of each bit is 1 if the corresponding elements"] # [doc = " of a and b are not equal. Otherwise, the value of each bit is 0."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cmpne < T , U > (a : T , b : U) -> < T as sealed :: VectorCmpNe < U > > :: Result where T : sealed :: VectorCmpNe < U > , { a . vec_cmpne (b) }
}

macro_rules! vec_cmpb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cmpb in module {}", module_path!());
    };
}

mkfn!{
    vec_cmpb_introspect!();
    # [doc = " Vector cmpb."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cmpb (a : vector_float , b : vector_float) -> vector_signed_int { sealed :: vec_vcmpbfp (a , b) }
}

macro_rules! vec_ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ceil in module {}", module_path!());
    };
}

mkfn!{
    vec_ceil_introspect!();
    # [doc = " Vector ceil."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_ceil (a : vector_float) -> vector_float { sealed :: vec_vceil (a) }
}

macro_rules! vec_avg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_avg in module {}", module_path!());
    };
}

mkfn!{
    vec_avg_introspect!();
    # [doc = " Vector avg."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_avg < T , U > (a : T , b : U) -> < T as sealed :: VectorAvg < U > > :: Result where T : sealed :: VectorAvg < U > , { a . vec_avg (b) }
}

macro_rules! vec_andc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_andc in module {}", module_path!());
    };
}

mkfn!{
    vec_andc_introspect!();
    # [doc = " Vector andc."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_andc < T , U > (a : T , b : U) -> < T as sealed :: VectorAndc < U > > :: Result where T : sealed :: VectorAndc < U > , { a . vec_andc (b) }
}

macro_rules! vec_orc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_orc in module {}", module_path!());
    };
}

mkfn!{
    vec_orc_introspect!();
    # [doc = " Vector OR with Complement"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Performs a bitwise OR of the first vector with the bitwise-complemented second vector."] # [doc = ""] # [doc = " ## Result value"] # [doc = " r is the bitwise OR of a and the bitwise complement of b."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_orc < T , U > (a : T , b : U) -> < T as sealed :: VectorOrc < U > > :: Result where T : sealed :: VectorOrc < U > , { a . vec_orc (b) }
}

macro_rules! vec_and_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_and in module {}", module_path!());
    };
}

mkfn!{
    vec_and_introspect!();
    # [doc = " Vector and."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_and < T , U > (a : T , b : U) -> < T as sealed :: VectorAnd < U > > :: Result where T : sealed :: VectorAnd < U > , { a . vec_and (b) }
}

macro_rules! vec_or_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_or in module {}", module_path!());
    };
}

mkfn!{
    vec_or_introspect!();
    # [doc = " Vector or."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_or < T , U > (a : T , b : U) -> < T as sealed :: VectorOr < U > > :: Result where T : sealed :: VectorOr < U > , { a . vec_or (b) }
}

macro_rules! vec_nand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_nand in module {}", module_path!());
    };
}

mkfn!{
    vec_nand_introspect!();
    # [doc = " Vector NAND"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Performs a bitwise NAND of two vectors."] # [doc = ""] # [doc = " ## Result value"] # [doc = " r is the bitwise NAND of a and b."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_nand < T , U > (a : T , b : U) -> < T as sealed :: VectorNand < U > > :: Result where T : sealed :: VectorNand < U > , { a . vec_nand (b) }
}

macro_rules! vec_nor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_nor in module {}", module_path!());
    };
}

mkfn!{
    vec_nor_introspect!();
    # [doc = " Vector nor."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_nor < T , U > (a : T , b : U) -> < T as sealed :: VectorNor < U > > :: Result where T : sealed :: VectorNor < U > , { a . vec_nor (b) }
}

macro_rules! vec_xor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_xor in module {}", module_path!());
    };
}

mkfn!{
    vec_xor_introspect!();
    # [doc = " Vector xor."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_xor < T , U > (a : T , b : U) -> < T as sealed :: VectorXor < U > > :: Result where T : sealed :: VectorXor < U > , { a . vec_xor (b) }
}

macro_rules! vec_adds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_adds in module {}", module_path!());
    };
}

mkfn!{
    vec_adds_introspect!();
    # [doc = " Vector adds."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_adds < T , U > (a : T , b : U) -> < T as sealed :: VectorAdds < U > > :: Result where T : sealed :: VectorAdds < U > , { a . vec_adds (b) }
}

macro_rules! vec_addc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_addc in module {}", module_path!());
    };
}

mkfn!{
    vec_addc_introspect!();
    # [doc = " Vector addc."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_addc (a : vector_unsigned_int , b : vector_unsigned_int) -> vector_unsigned_int { sealed :: vec_vaddcuw (a , b) }
}

macro_rules! vec_abs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_abs in module {}", module_path!());
    };
}

mkfn!{
    vec_abs_introspect!();
    # [doc = " Vector abs."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_abs < T > (a : T) -> T where T : sealed :: VectorAbs , { a . vec_abs () }
}

macro_rules! vec_abss_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_abss in module {}", module_path!());
    };
}

mkfn!{
    vec_abss_introspect!();
    # [doc = " Vector abss."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_abss < T > (a : T) -> T where T : sealed :: VectorAbss , { a . vec_abss () }
}

macro_rules! vec_rl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_rl in module {}", module_path!());
    };
}

mkfn!{
    vec_rl_introspect!();
    # [doc = " Vector Rotate Left"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Rotates each element of a vector left by a given number of bits."] # [doc = ""] # [doc = " ## Result value"] # [doc = " Each element of r is obtained by rotating the corresponding element of a left by"] # [doc = " the number of bits specified by the corresponding element of b."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_rl < T > (a : T , b : < T as sealed :: VectorRl > :: Shift) -> T where T : sealed :: VectorRl , { a . vec_rl (b) }
}

macro_rules! vec_round_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_round in module {}", module_path!());
    };
}

mkfn!{
    vec_round_introspect!();
    # [doc = " Vector Round"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Returns a vector containing the rounded values of the corresponding elements of the"] # [doc = " source vector."] # [doc = ""] # [doc = " ## Result value"] # [doc = " Each element of r contains the value of the corresponding element of a, rounded"] # [doc = " to the nearest representable floating-point integer, using IEEE round-to-nearest"] # [doc = " rounding."] # [doc = " The current floating-point rounding mode is ignored."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_round < T > (a : T) -> T where T : sealed :: VectorRound , { a . vec_round () }
}

macro_rules! vec_splat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_splat in module {}", module_path!());
    };
}

mkfn!{
    vec_splat_introspect!();
    # [doc = " Vector Splat"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_splat < T , const IMM : u32 > (a : T) -> T where T : sealed :: VectorSplat , { a . vec_splat :: < IMM > () }
}
mkitem!{splat ! { vec_splat_u8 , u8 , u8x16 [vspltisb / xxspltib , "Vector Splat to Unsigned Byte"] }}
mkitem!{splat ! { vec_splat_s8 , i8 , i8x16 [vspltisb / xxspltib , "Vector Splat to Signed Byte"] }}
mkitem!{splat ! { vec_splat_u16 , u16 , u16x8 [vspltish , "Vector Splat to Unsigned Halfword"] }}
mkitem!{splat ! { vec_splat_s16 , i16 , i16x8 [vspltish , "Vector Splat to Signed Halfword"] }}
mkitem!{splat ! { vec_splat_u32 , u32 , u32x4 [vspltisw , "Vector Splat to Unsigned Word"] }}
mkitem!{splat ! { vec_splat_s32 , i32 , i32x4 [vspltisw , "Vector Splat to Signed Word"] }}

macro_rules! vec_splats_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_splats in module {}", module_path!());
    };
}

mkfn!{
    vec_splats_introspect!();
    # [doc = " Vector splats."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_splats < T > (a : T) -> < T as sealed :: VectorSplats > :: Result where T : sealed :: VectorSplats , { a . vec_splats () }
}

macro_rules! vec_sub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sub in module {}", module_path!());
    };
}

mkfn!{
    vec_sub_introspect!();
    # [doc = " Vector sub."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sub < T , U > (a : T , b : U) -> < T as sealed :: VectorSub < U > > :: Result where T : sealed :: VectorSub < U > , { a . vec_sub (b) }
}

macro_rules! vec_subc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_subc in module {}", module_path!());
    };
}

mkfn!{
    vec_subc_introspect!();
    # [doc = " Vector Subtract Carryout"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Returns a vector wherein each element contains the carry produced by subtracting the"] # [doc = " corresponding elements of the two source vectors."] # [doc = ""] # [doc = " ## Result value"] # [doc = " The value of each element of r is the complement of the carry produced by subtract- ing the"] # [doc = " value of the corresponding element of b from the value of the corresponding element of a. The"] # [doc = " value is 0 if a borrow occurred, or 1 if no borrow occurred."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_subc < T , U > (a : T , b : U) -> < T as sealed :: VectorSubc < U > > :: Result where T : sealed :: VectorSubc < U > , { a . vec_subc (b) }
}

macro_rules! vec_subs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_subs in module {}", module_path!());
    };
}

mkfn!{
    vec_subs_introspect!();
    # [doc = " Vector subs."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_subs < T , U > (a : T , b : U) -> < T as sealed :: VectorSubs < U > > :: Result where T : sealed :: VectorSubs < U > , { a . vec_subs (b) }
}

macro_rules! vec_min_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_min in module {}", module_path!());
    };
}

mkfn!{
    vec_min_introspect!();
    # [doc = " Vector min."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_min < T , U > (a : T , b : U) -> < T as sealed :: VectorMin < U > > :: Result where T : sealed :: VectorMin < U > , { a . vec_min (b) }
}

macro_rules! vec_max_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_max in module {}", module_path!());
    };
}

mkfn!{
    vec_max_introspect!();
    # [doc = " Vector max."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_max < T , U > (a : T , b : U) -> < T as sealed :: VectorMax < U > > :: Result where T : sealed :: VectorMax < U > , { a . vec_max (b) }
}

macro_rules! vec_mfvscr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mfvscr in module {}", module_path!());
    };
}

mkfn!{
    vec_mfvscr_introspect!();
    # [doc = " Move From Vector Status and Control Register."] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (mfvscr))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mfvscr () -> vector_unsigned_short { mfvscr () }
}

macro_rules! vec_add_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add in module {}", module_path!());
    };
}

mkfn!{
    vec_add_introspect!();
    # [doc = " Vector add."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_add < T , U > (a : T , b : U) -> < T as sealed :: VectorAdd < U > > :: Result where T : sealed :: VectorAdd < U > , { a . vec_add (b) }
}

macro_rules! vec_adde_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_adde in module {}", module_path!());
    };
}

mkfn!{
    vec_adde_introspect!();
    # [doc = " Vector Add Extended"] # [doc = ""] # [doc = " ## Result value"] # [doc = " The value of each element of r is produced by adding the corresponding elements of"] # [doc = " a and b with a carry specified in the corresponding element of c (1 if there is a carry, 0"] # [doc = " otherwise)."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_adde < T > (a : T , b : T , c : T) -> T where T : sealed :: VectorAdde , { a . vec_adde (b , c) }
}

macro_rules! vec_ctf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ctf in module {}", module_path!());
    };
}

mkfn!{
    vec_ctf_introspect!();
    # [doc = " Vector Convert to Floating-Point"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_ctf < const IMM5 : i32 , T > (a : T) -> vector_float where T : sealed :: VectorCtf , { a . vec_ctf :: < IMM5 > () }
}

macro_rules! vec_cts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cts in module {}", module_path!());
    };
}

mkfn!{
    vec_cts_introspect!();
    # [doc = " Vector Convert to Signed Integer"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vctsxs , IMM5 = 1))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cts < const IMM5 : i32 > (a : vector_float) -> vector_signed_int { static_assert_uimm_bits ! (IMM5 , 5) ; vctsxs (a , IMM5) }
}

macro_rules! vec_ctu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ctu in module {}", module_path!());
    };
}

mkfn!{
    vec_ctu_introspect!();
    # [doc = " Vector Convert to Unsigned Integer"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vctuxs , IMM5 = 1))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_ctu < const IMM5 : i32 > (a : vector_float) -> vector_unsigned_int { static_assert_uimm_bits ! (IMM5 , 5) ; vctuxs (a , IMM5) }
}
mkmod!{endian, { 
                getname!(endian);
                getsrc!(endian);
                getpath!(endian);
                get_deps!(endian);
                get_crates!(endian);
                mkinclude!(endian);
                mkuse!{use super :: * ;}

macro_rules! vec_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_perm in module {}", module_path!());
    };
}

mkfn!{
    vec_perm_introspect!();
    # [doc = " Vector permute."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_perm < T > (a : T , b : T , c : vector_unsigned_char) -> T where T : sealed :: VectorPerm , { let d = transmute (u8x16 :: new (255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 ,)) ; let c = simd_xor (c , d) ; b . vec_vperm (a , c) }
}

macro_rules! vec_sum2s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sum2s in module {}", module_path!());
    };
}

mkfn!{
    vec_sum2s_introspect!();
    # [doc = " Vector Sum Across Partial (1/2) Saturated"] # [inline] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] # [target_feature (enable = "altivec")] pub unsafe fn vec_sum2s (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int { let flip = transmute (u8x16 :: new (4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 12 , 13 , 14 , 15 , 8 , 9 , 10 , 11 ,)) ; let b = vec_perm (b , b , flip) ; let c = vsum2sws (a , b) ; vec_perm (c , c , flip) }
}

macro_rules! vec_mule_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mule in module {}", module_path!());
    };
}

mkfn!{
    vec_mule_introspect!();
    # [doc = " Vector Multiply Even"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mule < T , U > (a : T , b : T) -> U where T : sealed :: VectorMulo < U > , { a . vec_mulo (b) }
}

macro_rules! vec_mulo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mulo in module {}", module_path!());
    };
}

mkfn!{
    vec_mulo_introspect!();
    # [doc = " Vector Multiply Odd"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mulo < T , U > (a : T , b : T) -> U where T : sealed :: VectorMule < U > , { a . vec_mule (b) }
} 
            }}

macro_rules! vec_mul_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mul in module {}", module_path!());
    };
}

mkfn!{
    vec_mul_introspect!();
    # [doc = " Vector Multiply"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Compute the products of corresponding elements of two vectors."] # [doc = ""] # [doc = " ## Result value"] # [doc = " Each element of r receives the product of the corresponding elements of a and b."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mul < T > (a : T , b : T) -> T where T : sealed :: VectorMul , { a . vec_mul (b) }
}

macro_rules! vec_madds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_madds in module {}", module_path!());
    };
}

mkfn!{
    vec_madds_introspect!();
    # [doc = " Vector Multiply Add Saturated"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmhaddshs))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_madds (a : vector_signed_short , b : vector_signed_short , c : vector_signed_short ,) -> vector_signed_short { vmhaddshs (a , b , c) }
}

macro_rules! vec_mladd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mladd in module {}", module_path!());
    };
}

mkfn!{
    vec_mladd_introspect!();
    # [doc = " Vector Multiply Low and Add Unsigned Half Word"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mladd < T , U > (a : T , b : U , c : U) -> < T as sealed :: VectorMladd < U > > :: Result where T : sealed :: VectorMladd < U > , { a . vec_mladd (b , c) }
}

macro_rules! vec_mradds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mradds in module {}", module_path!());
    };
}

mkfn!{
    vec_mradds_introspect!();
    # [doc = " Vector Multiply Round and Add Saturated"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr (vmhraddshs))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mradds (a : vector_signed_short , b : vector_signed_short , c : vector_signed_short ,) -> vector_signed_short { vmhraddshs (a , b , c) }
}

macro_rules! vec_msum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_msum in module {}", module_path!());
    };
}

mkfn!{
    vec_msum_introspect!();
    # [doc = " Vector Multiply Sum"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_msum < T , B , U > (a : T , b : B , c : U) -> U where T : sealed :: VectorMsum < B , U > , { a . vec_msum (b , c) }
}

macro_rules! vec_msums_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_msums in module {}", module_path!());
    };
}

mkfn!{
    vec_msums_introspect!();
    # [doc = " Vector Multiply Sum Saturated"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_msums < T , U > (a : T , b : T , c : U) -> U where T : sealed :: VectorMsums < U > , { a . vec_msums (b , c) }
}

macro_rules! vec_madd_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_madd in module {}", module_path!());
    };
}

mkfn!{
    vec_madd_introspect!();
    # [doc = " Vector Multiply Add"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_madd (a : vector_float , b : vector_float , c : vector_float) -> vector_float { sealed :: vec_vmaddfp (a , b , c) }
}

macro_rules! vec_nmsub_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_nmsub in module {}", module_path!());
    };
}

mkfn!{
    vec_nmsub_introspect!();
    # [doc = " Vector Negative Multiply Subtract"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_nmsub (a : vector_float , b : vector_float , c : vector_float) -> vector_float { vnmsubfp (a , b , c) }
}

macro_rules! vec_sel_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sel in module {}", module_path!());
    };
}

mkfn!{
    vec_sel_introspect!();
    # [doc = " Vector Select"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Returns a vector selecting bits from two source vectors depending on the corresponding"] # [doc = " bit values of a third source vector."] # [doc = ""] # [doc = " ## Result value"] # [doc = " Each bit of r has the value of the corresponding bit of a if the corresponding"] # [doc = " bit of c is 0. Otherwise, the bit of r has the value of the corresponding bit of b."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sel < T , U > (a : T , b : T , c : U) -> T where T : sealed :: VectorSel < U > , { a . vec_sel (b , c) }
}

macro_rules! vec_sum4s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sum4s in module {}", module_path!());
    };
}

mkfn!{
    vec_sum4s_introspect!();
    # [doc = " Vector Sum Across Partial (1/4) Saturated"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sum4s < T , U > (a : T , b : U) -> U where T : sealed :: VectorSum4s < U > , { a . vec_sum4s (b) }
}

macro_rules! vec_all_eq_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_eq in module {}", module_path!());
    };
}

mkfn!{
    vec_all_eq_introspect!();
    # [doc = " Vector All Elements Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_eq < T , U > (a : T , b : U) -> < T as sealed :: VectorAllEq < U > > :: Result where T : sealed :: VectorAllEq < U > , { a . vec_all_eq (b) }
}

macro_rules! vec_any_eq_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_eq in module {}", module_path!());
    };
}

mkfn!{
    vec_any_eq_introspect!();
    # [doc = " Vector All Elements Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_eq < T , U > (a : T , b : U) -> < T as sealed :: VectorAnyEq < U > > :: Result where T : sealed :: VectorAnyEq < U > , { a . vec_any_eq (b) }
}

macro_rules! vec_all_ge_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_ge in module {}", module_path!());
    };
}

mkfn!{
    vec_all_ge_introspect!();
    # [doc = " Vector All Elements Greater or Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_ge < T , U > (a : T , b : U) -> < T as sealed :: VectorAllGe < U > > :: Result where T : sealed :: VectorAllGe < U > , { a . vec_all_ge (b) }
}

macro_rules! vec_any_ge_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_ge in module {}", module_path!());
    };
}

mkfn!{
    vec_any_ge_introspect!();
    # [doc = " Vector Any Element Greater or Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_ge < T , U > (a : T , b : U) -> < T as sealed :: VectorAnyGe < U > > :: Result where T : sealed :: VectorAnyGe < U > , { a . vec_any_ge (b) }
}

macro_rules! vec_all_gt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_gt in module {}", module_path!());
    };
}

mkfn!{
    vec_all_gt_introspect!();
    # [doc = " Vector All Elements Greater Than"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_gt < T , U > (a : T , b : U) -> < T as sealed :: VectorAllGt < U > > :: Result where T : sealed :: VectorAllGt < U > , { a . vec_all_gt (b) }
}

macro_rules! vec_any_gt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_gt in module {}", module_path!());
    };
}

mkfn!{
    vec_any_gt_introspect!();
    # [doc = " Vector Any Element Greater Than"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_gt < T , U > (a : T , b : U) -> < T as sealed :: VectorAnyGt < U > > :: Result where T : sealed :: VectorAnyGt < U > , { a . vec_any_gt (b) }
}

macro_rules! vec_all_in_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_in in module {}", module_path!());
    };
}

mkfn!{
    vec_all_in_introspect!();
    # [doc = " Vector All In"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpbfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_in (a : vector_float , b : vector_float) -> bool { vcmpbfp_p (0 , a , b) != 0 }
}

macro_rules! vec_all_le_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_le in module {}", module_path!());
    };
}

mkfn!{
    vec_all_le_introspect!();
    # [doc = " Vector All Elements Less Than or Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_le < T , U > (a : U , b : T) -> < T as sealed :: VectorAllGe < U > > :: Result where T : sealed :: VectorAllGe < U > , { b . vec_all_ge (a) }
}

macro_rules! vec_any_le_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_le in module {}", module_path!());
    };
}

mkfn!{
    vec_any_le_introspect!();
    # [doc = " Vector Any Element Less Than or Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_le < T , U > (a : U , b : T) -> < T as sealed :: VectorAnyGe < U > > :: Result where T : sealed :: VectorAnyGe < U > , { b . vec_any_ge (a) }
}

macro_rules! vec_all_lt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_lt in module {}", module_path!());
    };
}

mkfn!{
    vec_all_lt_introspect!();
    # [doc = " Vector All Elements Less Than"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_lt < T , U > (a : U , b : T) -> < T as sealed :: VectorAllGt < U > > :: Result where T : sealed :: VectorAllGt < U > , { b . vec_all_gt (a) }
}

macro_rules! vec_any_lt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_lt in module {}", module_path!());
    };
}

mkfn!{
    vec_any_lt_introspect!();
    # [doc = " Vector Any Element Less Than"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_lt < T , U > (a : U , b : T) -> < T as sealed :: VectorAnyGt < U > > :: Result where T : sealed :: VectorAnyGt < U > , { b . vec_any_gt (a) }
}

macro_rules! vec_all_nan_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_nan in module {}", module_path!());
    };
}

mkfn!{
    vec_all_nan_introspect!();
    # [doc = " All Elements Not a Number"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpeqfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_nan (a : vector_float) -> bool { vcmpeqfp_p (0 , a , a) != 0 }
}

macro_rules! vec_any_nan_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_nan in module {}", module_path!());
    };
}

mkfn!{
    vec_any_nan_introspect!();
    # [doc = " Any Elements Not a Number"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpeqfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_nan (a : vector_float) -> bool { vcmpeqfp_p (3 , a , a) != 0 }
}

macro_rules! vec_all_ne_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_ne in module {}", module_path!());
    };
}

mkfn!{
    vec_all_ne_introspect!();
    # [doc = " Vector All Elements Not Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_ne < T , U > (a : T , b : U) -> < T as sealed :: VectorAllNe < U > > :: Result where T : sealed :: VectorAllNe < U > , { a . vec_all_ne (b) }
}

macro_rules! vec_any_ne_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_ne in module {}", module_path!());
    };
}

mkfn!{
    vec_any_ne_introspect!();
    # [doc = " Vector Any Elements Not Equal"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_ne < T , U > (a : T , b : U) -> < T as sealed :: VectorAnyNe < U > > :: Result where T : sealed :: VectorAnyNe < U > , { a . vec_any_ne (b) }
}

macro_rules! vec_all_nge_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_nge in module {}", module_path!());
    };
}

mkfn!{
    vec_all_nge_introspect!();
    # [doc = " All Elements Not Greater Than or Equal"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgefp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_nge (a : vector_float , b : vector_float) -> bool { vcmpgefp_p (0 , a , b) != 0 }
}

macro_rules! vec_all_ngt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_ngt in module {}", module_path!());
    };
}

mkfn!{
    vec_all_ngt_introspect!();
    # [doc = " All Elements Not Greater Than"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgtfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_ngt (a : vector_float , b : vector_float) -> bool { vcmpgtfp_p (0 , a , b) != 0 }
}

macro_rules! vec_all_nle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_nle in module {}", module_path!());
    };
}

mkfn!{
    vec_all_nle_introspect!();
    # [doc = " All Elements Not Less Than or Equal"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgefp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_nle (a : vector_float , b : vector_float) -> bool { vcmpgefp_p (0 , b , a) != 0 }
}

macro_rules! vec_all_nlt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_nlt in module {}", module_path!());
    };
}

mkfn!{
    vec_all_nlt_introspect!();
    # [doc = " All Elements Not Less Than"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgtfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_nlt (a : vector_float , b : vector_float) -> bool { vcmpgtfp_p (0 , b , a) != 0 }
}

macro_rules! vec_all_numeric_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_all_numeric in module {}", module_path!());
    };
}

mkfn!{
    vec_all_numeric_introspect!();
    # [doc = " All Elements Numeric"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgefp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_all_numeric (a : vector_float) -> bool { vcmpgefp_p (2 , a , a) != 0 }
}

macro_rules! vec_any_nge_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_nge in module {}", module_path!());
    };
}

mkfn!{
    vec_any_nge_introspect!();
    # [doc = " Any Elements Not Greater Than or Equal"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgefp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_nge (a : vector_float , b : vector_float) -> bool { vcmpgefp_p (3 , a , b) != 0 }
}

macro_rules! vec_any_ngt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_ngt in module {}", module_path!());
    };
}

mkfn!{
    vec_any_ngt_introspect!();
    # [doc = " Any Elements Not Greater Than"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgtfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_ngt (a : vector_float , b : vector_float) -> bool { vcmpgtfp_p (3 , a , b) != 0 }
}

macro_rules! vec_any_nle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_nle in module {}", module_path!());
    };
}

mkfn!{
    vec_any_nle_introspect!();
    # [doc = " Any Elements Not Less Than or Equal"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgefp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_nle (a : vector_float , b : vector_float) -> bool { vcmpgefp_p (3 , b , a) != 0 }
}

macro_rules! vec_any_nlt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_nlt in module {}", module_path!());
    };
}

mkfn!{
    vec_any_nlt_introspect!();
    # [doc = " Any Elements Not Less Than"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgtfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_nlt (a : vector_float , b : vector_float) -> bool { vcmpgtfp_p (3 , b , a) != 0 }
}

macro_rules! vec_any_numeric_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_numeric in module {}", module_path!());
    };
}

mkfn!{
    vec_any_numeric_introspect!();
    # [doc = " Any Elements Numeric"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpgefp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_numeric (a : vector_float) -> bool { vcmpgefp_p (1 , a , a) != 0 }
}

macro_rules! vec_cntlz_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_cntlz in module {}", module_path!());
    };
}

mkfn!{
    vec_cntlz_introspect!();
    # [doc = " Vector Count Leading Zeros"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Returns a vector containing the number of most-significant bits equal to zero of each"] # [doc = " corresponding element of the source vector."] # [doc = ""] # [doc = " ## Result value"] # [doc = " The value of each element of r is set to the number of leading zeros of the"] # [doc = " corresponding element of a."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_cntlz < T > (a : T) -> T where T : sealed :: VectorCntlz , { a . vec_cntlz () }
}

macro_rules! vec_any_out_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_any_out in module {}", module_path!());
    };
}

mkfn!{
    vec_any_out_introspect!();
    # [doc = " Any Element Out of Bounds"] # [inline] # [target_feature (enable = "altivec")] # [cfg_attr (test , assert_instr ("vcmpeqfp."))] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_any_out (a : vector_float) -> bool { vcmpeqfp_p (1 , a , a) != 0 }
}
mkmod!{endian, { 
                getname!(endian);
                getsrc!(endian);
                getpath!(endian);
                get_deps!(endian);
                get_crates!(endian);
                mkinclude!(endian);
                mkuse!{use super :: * ;}

macro_rules! vec_perm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_perm in module {}", module_path!());
    };
}

mkfn!{
    vec_perm_introspect!();
    # [doc = " Vector permute."] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_perm < T > (a : T , b : T , c : vector_unsigned_char) -> T where T : sealed :: VectorPerm , { a . vec_vperm (b , c) }
}

macro_rules! vec_sum2s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_sum2s in module {}", module_path!());
    };
}

mkfn!{
    vec_sum2s_introspect!();
    # [doc = " Vector Sum Across Partial (1/2) Saturated"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_sum2s (a : vector_signed_int , b : vector_signed_int) -> vector_signed_int { vsum2sws (a , b) }
}

macro_rules! vec_mule_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mule in module {}", module_path!());
    };
}

mkfn!{
    vec_mule_introspect!();
    # [doc = " Vector Multiply Even"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mule < T , U > (a : T , b : T) -> U where T : sealed :: VectorMule < U > , { a . vec_mule (b) }
}

macro_rules! vec_mulo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_mulo in module {}", module_path!());
    };
}

mkfn!{
    vec_mulo_introspect!();
    # [doc = " Vector Multiply Odd"] # [inline] # [target_feature (enable = "altivec")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_mulo < T , U > (a : T , b : T) -> U where T : sealed :: VectorMulo < U > , { a . vec_mulo (b) }
} 
            }}
mkuse!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub use self :: endian :: * ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}
mkuse!{use std :: mem :: transmute ;}
mkuse!{use crate :: core_arch :: simd :: * ;}
mkuse!{use stdarch_test :: simd_test ;}
mkitem!{macro_rules ! test_vec_2 { { $ name : ident , $ fn : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { test_vec_2 ! { $ name , $ fn , $ ty -> $ ty , [$ ($ a) ,+] , [$ ($ b) ,+] , [$ ($ d) ,+] } } ; { $ name : ident , $ fn : ident , $ ty : ident -> $ ty_out : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ a) ,+)) ; let b : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ b) ,+)) ; let d = $ ty_out :: new ($ ($ d) ,+) ; let r : $ ty_out = transmute ($ fn (a , b)) ; assert_eq ! (d , r) ; } } ; { $ name : ident , $ fn : ident , $ ty : ident -> $ ty_out : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , $ d : expr } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ a) ,+)) ; let b : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ b) ,+)) ; let r : $ ty_out = transmute ($ fn (a , b)) ; assert_eq ! ($ d , r) ; } } }}
mkitem!{macro_rules ! test_vec_1 { { $ name : ident , $ fn : ident , f32x4 , [$ ($ a : expr) ,+] , ~ [$ ($ d : expr) ,+] } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : vector_float = transmute (f32x4 :: new ($ ($ a) ,+)) ; let d : vector_float = transmute (f32x4 :: new ($ ($ d) ,+)) ; let r = transmute (vec_cmple (vec_abs (vec_sub ($ fn (a) , d)) , vec_splats (f32 :: EPSILON))) ; let e = m32x4 :: new (true , true , true , true) ; assert_eq ! (e , r) ; } } ; { $ name : ident , $ fn : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ d : expr) ,+] } => { test_vec_1 ! { $ name , $ fn , $ ty -> $ ty , [$ ($ a) ,+] , [$ ($ d) ,+] } } ; { $ name : ident , $ fn : ident , $ ty : ident -> $ ty_out : ident , [$ ($ a : expr) ,+] , [$ ($ d : expr) ,+] } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ a) ,+)) ; let d = $ ty_out :: new ($ ($ d) ,+) ; let r : $ ty_out = transmute ($ fn (a)) ; assert_eq ! (d , r) ; } } }}

macro_rules! test_vec_ld_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_ld in module {}", module_path!());
    };
}

mkfn!{
    test_vec_ld_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_ld () { let pat = [u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) , u8x16 :: new (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ,] ; for off in 0 .. 16 { let v : u8x16 = transmute (vec_ld (0 , (pat . as_ptr () as * const u8) . offset (off))) ; assert_eq ! (v , u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15)) ; } for off in 16 .. 32 { let v : u8x16 = transmute (vec_ld (0 , (pat . as_ptr () as * const u8) . offset (off))) ; assert_eq ! (v , u8x16 :: new (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31)) ; } }
}

macro_rules! test_vec_xl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_xl in module {}", module_path!());
    };
}

mkfn!{
    test_vec_xl_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_xl () { let pat = [u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) , u8x16 :: new (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ,] ; for off in 0 .. 16 { let val : u8x16 = transmute (vec_xl (0 , (pat . as_ptr () as * const u8) . offset (off))) ; for i in 0 .. 16 { let v = val . extract (i) ; assert_eq ! (off as usize + i , v as usize) ; } } }
}

macro_rules! test_vec_xst_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_xst in module {}", module_path!());
    };
}

mkfn!{
    test_vec_xst_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_xst () { let v : vector_unsigned_char = transmute (u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,)) ; for off in 0 .. 16 { let mut buf = [0u8 ; 32] ; vec_xst (v , 0 , (buf . as_mut_ptr () as * mut u8) . offset (off)) ; for i in 0 .. 16 { assert_eq ! (i as u8 , buf [off as usize ..] [i]) ; } } }
}

macro_rules! test_vec_ldl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_ldl in module {}", module_path!());
    };
}

mkfn!{
    test_vec_ldl_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_ldl () { let pat = [u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15) , u8x16 :: new (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31 ,) ,] ; for off in 0 .. 16 { let v : u8x16 = transmute (vec_ldl (0 , (pat . as_ptr () as * const u8) . offset (off))) ; assert_eq ! (v , u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15)) ; } for off in 16 .. 32 { let v : u8x16 = transmute (vec_ldl (0 , (pat . as_ptr () as * const u8) . offset (off))) ; assert_eq ! (v , u8x16 :: new (16 , 17 , 18 , 19 , 20 , 21 , 22 , 23 , 24 , 25 , 26 , 27 , 28 , 29 , 30 , 31)) ; } }
}

macro_rules! test_vec_lde_u8_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_lde_u8 in module {}", module_path!());
    };
}

mkfn!{
    test_vec_lde_u8_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_lde_u8 () { let pat = [u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15 ,)] ; for off in 0 .. 16 { let v : u8x16 = transmute (vec_lde (off , pat . as_ptr () as * const u8)) ; assert_eq ! (off as u8 , v . extract (off as _)) ; } }
}

macro_rules! test_vec_lde_u16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_lde_u16 in module {}", module_path!());
    };
}

mkfn!{
    test_vec_lde_u16_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_lde_u16 () { let pat = [u16x8 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)] ; for off in 0 .. 8 { let v : u16x8 = transmute (vec_lde (off * 2 , pat . as_ptr () as * const u16)) ; assert_eq ! (off as u16 , v . extract (off as _)) ; } }
}

macro_rules! test_vec_lde_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_lde_u32 in module {}", module_path!());
    };
}

mkfn!{
    test_vec_lde_u32_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_lde_u32 () { let pat = [u32x4 :: new (0 , 1 , 2 , 3)] ; for off in 0 .. 4 { let v : u32x4 = transmute (vec_lde (off * 4 , pat . as_ptr () as * const u32)) ; assert_eq ! (off as u32 , v . extract (off as _)) ; } }
}
mkitem!{test_vec_1 ! { test_vec_floor , vec_floor , f32x4 , [1.1 , 1.9 , - 0.5 , - 0.9] , [1.0 , 1.0 , - 1.0 , - 1.0] }}
mkitem!{test_vec_1 ! { test_vec_expte , vec_expte , f32x4 , [0.0 , 2.0 , 2.0 , - 1.0] , ~ [1.0 , 4.0 , 4.0 , 0.5] }}
mkitem!{test_vec_2 ! { test_vec_cmpgt_i8 , vec_cmpgt , i8x16 -> m8x16 , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [true , false , true , false , false , false , false , false , false , false , false , false , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpgt_u8 , vec_cmpgt , u8x16 -> m8x16 , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [true , true , false , false , false , false , false , false , false , false , false , false , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpgt_i16 , vec_cmpgt , i16x8 -> m16x8 , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , [true , false , true , false , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpgt_u16 , vec_cmpgt , u16x8 -> m16x8 , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , [true , true , false , false , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpgt_i32 , vec_cmpgt , i32x4 -> m32x4 , [1 , - 1 , 0 , 0] , [0 , - 1 , 0 , 1] , [true , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpgt_u32 , vec_cmpgt , u32x4 -> m32x4 , [1 , 255 , 0 , 0] , [0 , 255 , 0 , 1] , [true , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpge , vec_cmpge , f32x4 -> m32x4 , [0.1 , - 0.1 , 0.0 , 0.99] , [0.1 , 0.0 , 0.1 , 1.0] , [true , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpeq_i8 , vec_cmpeq , i8x16 -> m8x16 , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [false , false , false , false , true , true , true , true , true , true , true , true , true , true , true , true] }}
mkitem!{test_vec_2 ! { test_vec_cmpeq_u8 , vec_cmpeq , u8x16 -> m8x16 , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [false , false , false , false , true , true , true , true , true , true , true , true , true , true , true , true] }}
mkitem!{test_vec_2 ! { test_vec_cmpeq_i16 , vec_cmpeq , i16x8 -> m16x8 , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , [false , false , false , false , true , true , true , true] }}
mkitem!{test_vec_2 ! { test_vec_cmpeq_u16 , vec_cmpeq , u16x8 -> m16x8 , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , [false , false , false , false , true , true , true , true] }}
mkitem!{test_vec_2 ! { test_vec_cmpeq_i32 , vec_cmpeq , i32x4 -> m32x4 , [1 , - 1 , 0 , 0] , [0 , - 1 , 0 , 1] , [false , true , true , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpeq_u32 , vec_cmpeq , u32x4 -> m32x4 , [1 , 255 , 0 , 0] , [0 , 255 , 0 , 1] , [false , true , true , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpne_i8 , vec_cmpne , i8x16 -> m8x16 , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [true , true , true , true , false , false , false , false , false , false , false , false , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpne_u8 , vec_cmpne , u8x16 -> m8x16 , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [true , true , true , true , false , false , false , false , false , false , false , false , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpne_i16 , vec_cmpne , i16x8 -> m16x8 , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , [true , true , true , true , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpne_u16 , vec_cmpne , u16x8 -> m16x8 , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , [true , true , true , true , false , false , false , false] }}
mkitem!{test_vec_2 ! { test_vec_cmpne_i32 , vec_cmpne , i32x4 -> m32x4 , [1 , - 1 , 0 , 0] , [0 , - 1 , 0 , 1] , [true , false , false , true] }}
mkitem!{test_vec_2 ! { test_vec_cmpne_u32 , vec_cmpne , u32x4 -> m32x4 , [1 , 255 , 0 , 0] , [0 , 255 , 0 , 1] , [true , false , false , true] }}
mkitem!{test_vec_2 ! { test_vec_all_eq_i8_false , vec_all_eq , i8x16 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_eq_u8_false , vec_all_eq , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_eq_i16_false , vec_all_eq , i16x8 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_eq_u16_false , vec_all_eq , u16x8 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_eq_i32_false , vec_all_eq , i32x4 -> bool , [1 , - 1 , 0 , 0] , [0 , - 1 , 0 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_eq_u32_false , vec_all_eq , u32x4 -> bool , [1 , 255 , 0 , 0] , [0 , 255 , 0 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_eq_i8_true , vec_all_eq , i8x16 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_eq_u8_true , vec_all_eq , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_eq_i16_true , vec_all_eq , i16x8 -> bool , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_eq_u16_true , vec_all_eq , u16x8 -> bool , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_eq_i32_true , vec_all_eq , i32x4 -> bool , [1 , - 1 , 0 , 1] , [1 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_eq_u32_true , vec_all_eq , u32x4 -> bool , [1 , 255 , 0 , 1] , [1 , 255 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_eq_i8_false , vec_any_eq , i8x16 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_eq_u8_false , vec_any_eq , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_eq_i16_false , vec_any_eq , i16x8 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_eq_u16_false , vec_any_eq , u16x8 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_eq_i32_false , vec_any_eq , i32x4 -> bool , [1 , - 1 , 0 , 0] , [0 , - 2 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_eq_u32_false , vec_any_eq , u32x4 -> bool , [1 , 2 , 1 , 0] , [0 , 255 , 0 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_eq_i8_true , vec_any_eq , i8x16 -> bool , [1 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_eq_u8_true , vec_any_eq , u8x16 -> bool , [0 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_eq_i16_true , vec_any_eq , i16x8 -> bool , [0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_eq_u16_true , vec_any_eq , u16x8 -> bool , [0 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_eq_i32_true , vec_any_eq , i32x4 -> bool , [0 , - 1 , 0 , 1] , [1 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_eq_u32_true , vec_any_eq , u32x4 -> bool , [0 , 255 , 0 , 1] , [1 , 255 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ge_i8_false , vec_all_ge , i8x16 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ge_u8_false , vec_all_ge , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ge_i16_false , vec_all_ge , i16x8 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ge_u16_false , vec_all_ge , u16x8 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ge_i32_false , vec_all_ge , i32x4 -> bool , [1 , - 1 , 0 , 0] , [0 , - 1 , 0 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ge_u32_false , vec_all_ge , u32x4 -> bool , [1 , 255 , 0 , 0] , [0 , 255 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ge_i8_true , vec_all_ge , i8x16 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ge_u8_true , vec_all_ge , u8x16 -> bool , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ge_i16_true , vec_all_ge , i16x8 -> bool , [1 , - 1 , 42 , 0 , 0 , 0 , 0 , 0] , [1 , - 5 , 2 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ge_u16_true , vec_all_ge , u16x8 -> bool , [42 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [2 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ge_i32_true , vec_all_ge , i32x4 -> bool , [1 , - 1 , 0 , 1] , [0 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ge_u32_true , vec_all_ge , u32x4 -> bool , [1 , 255 , 0 , 1] , [1 , 254 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ge_i8_false , vec_any_ge , i8x16 -> bool , [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ge_u8_false , vec_any_ge , u8x16 -> bool , [1 , 254 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [42 , 255 , 255 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ge_i16_false , vec_any_ge , i16x8 -> bool , [1 , - 1 , - 2 , 0 , 0 , 0 , 0 , 0] , [2 , 0 , - 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ge_u16_false , vec_any_ge , u16x8 -> bool , [1 , 2 , 0 , 0 , 0 , 0 , 0 , 0] , [2 , 42 , 255 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ge_i32_false , vec_any_ge , i32x4 -> bool , [1 , - 1 , 0 , 0] , [2 , 0 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ge_u32_false , vec_any_ge , u32x4 -> bool , [1 , 2 , 1 , 0] , [4 , 255 , 4 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ge_i8_true , vec_any_ge , i8x16 -> bool , [1 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ge_u8_true , vec_any_ge , u8x16 -> bool , [0 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ge_i16_true , vec_any_ge , i16x8 -> bool , [0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ge_u16_true , vec_any_ge , u16x8 -> bool , [0 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ge_i32_true , vec_any_ge , i32x4 -> bool , [0 , - 1 , 0 , 1] , [1 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ge_u32_true , vec_any_ge , u32x4 -> bool , [0 , 255 , 0 , 1] , [1 , 255 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_gt_i8_false , vec_all_gt , i8x16 -> bool , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_gt_u8_false , vec_all_gt , u8x16 -> bool , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_gt_i16_false , vec_all_gt , i16x8 -> bool , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_gt_u16_false , vec_all_gt , u16x8 -> bool , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_gt_i32_false , vec_all_gt , i32x4 -> bool , [1 , - 1 , 0 , 0] , [0 , - 1 , 0 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_gt_u32_false , vec_all_gt , u32x4 -> bool , [1 , 255 , 0 , 0] , [0 , 255 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_gt_i8_true , vec_all_gt , i8x16 -> bool , [2 , 1 , - 1 , 1 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 2 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_gt_u8_true , vec_all_gt , u8x16 -> bool , [1 , 255 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , [0 , 254 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_gt_i16_true , vec_all_gt , i16x8 -> bool , [1 , - 1 , 42 , 1 , 1 , 1 , 1 , 1] , [0 , - 5 , 2 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_gt_u16_true , vec_all_gt , u16x8 -> bool , [42 , 255 , 1 , 1 , 1 , 1 , 1 , 1] , [2 , 254 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_gt_i32_true , vec_all_gt , i32x4 -> bool , [1 , - 1 , 1 , 1] , [0 , - 2 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_gt_u32_true , vec_all_gt , u32x4 -> bool , [1 , 255 , 1 , 1] , [0 , 254 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_gt_i8_false , vec_any_gt , i8x16 -> bool , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_gt_u8_false , vec_any_gt , u8x16 -> bool , [1 , 254 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [42 , 255 , 255 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_gt_i16_false , vec_any_gt , i16x8 -> bool , [1 , - 1 , - 2 , 0 , 0 , 0 , 0 , 0] , [2 , 0 , - 1 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_gt_u16_false , vec_any_gt , u16x8 -> bool , [1 , 2 , 0 , 0 , 0 , 0 , 0 , 0] , [2 , 42 , 255 , 1 , 1 , 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_gt_i32_false , vec_any_gt , i32x4 -> bool , [1 , - 1 , 0 , 0] , [2 , 0 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_gt_u32_false , vec_any_gt , u32x4 -> bool , [1 , 2 , 1 , 0] , [4 , 255 , 4 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_gt_i8_true , vec_any_gt , i8x16 -> bool , [1 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_gt_u8_true , vec_any_gt , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_gt_i16_true , vec_any_gt , i16x8 -> bool , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , [0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_gt_u16_true , vec_any_gt , u16x8 -> bool , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [0 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_gt_i32_true , vec_any_gt , i32x4 -> bool , [1 , - 1 , 0 , 1] , [0 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_gt_u32_true , vec_any_gt , u32x4 -> bool , [1 , 255 , 0 , 1] , [0 , 255 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_in_true , vec_all_in , f32x4 -> bool , [0.0 , - 0.1 , 0.0 , 0.0] , [0.1 , 0.2 , 0.0 , 0.0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_in_false , vec_all_in , f32x4 -> bool , [0.5 , 0.4 , - 0.5 , 0.8] , [0.1 , 0.4 , - 0.5 , 0.8] , false }}
mkitem!{test_vec_2 ! { test_vec_all_le_i8_false , vec_all_le , i8x16 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_le_u8_false , vec_all_le , u8x16 -> bool , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_le_i16_false , vec_all_le , i16x8 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_le_u16_false , vec_all_le , u16x8 -> bool , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_le_i32_false , vec_all_le , i32x4 -> bool , [0 , - 1 , 0 , 1] , [1 , - 1 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_le_u32_false , vec_all_le , u32x4 -> bool , [0 , 255 , 1 , 1] , [1 , 255 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_le_i8_true , vec_all_le , i8x16 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_le_u8_true , vec_all_le , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_le_i16_true , vec_all_le , i16x8 -> bool , [1 , - 5 , 2 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 42 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_le_u16_true , vec_all_le , u16x8 -> bool , [2 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [42 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_le_i32_true , vec_all_le , i32x4 -> bool , [0 , - 1 , 0 , 1] , [1 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_le_u32_true , vec_all_le , u32x4 -> bool , [1 , 254 , 0 , 0] , [1 , 255 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_le_i8_false , vec_any_le , i8x16 -> bool , [1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_le_u8_false , vec_any_le , u8x16 -> bool , [42 , 255 , 255 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , [1 , 254 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_le_i16_false , vec_any_le , i16x8 -> bool , [2 , 0 , - 1 , 1 , 1 , 1 , 1 , 1] , [1 , - 1 , - 2 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_le_u16_false , vec_any_le , u16x8 -> bool , [2 , 42 , 255 , 1 , 1 , 1 , 1 , 1] , [1 , 2 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_le_i32_false , vec_any_le , i32x4 -> bool , [2 , 0 , 1 , 1] , [1 , - 1 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_le_u32_false , vec_any_le , u32x4 -> bool , [4 , 255 , 4 , 1] , [1 , 2 , 1 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_le_i8_true , vec_any_le , i8x16 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_le_u8_true , vec_any_le , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_le_i16_true , vec_any_le , i16x8 -> bool , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , [0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_le_u16_true , vec_any_le , u16x8 -> bool , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [0 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_le_i32_true , vec_any_le , i32x4 -> bool , [1 , - 1 , 0 , 1] , [0 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_le_u32_true , vec_any_le , u32x4 -> bool , [1 , 255 , 0 , 1] , [0 , 255 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_lt_i8_false , vec_all_lt , i8x16 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_lt_u8_false , vec_all_lt , u8x16 -> bool , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_lt_i16_false , vec_all_lt , i16x8 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0] , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_lt_u16_false , vec_all_lt , u16x8 -> bool , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0] , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_lt_i32_false , vec_all_lt , i32x4 -> bool , [0 , - 1 , 0 , 1] , [1 , - 1 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_lt_u32_false , vec_all_lt , u32x4 -> bool , [0 , 255 , 1 , 1] , [1 , 255 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_lt_i8_true , vec_all_lt , i8x16 -> bool , [0 , 0 , - 2 , 0 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1 , - 1] , [2 , 1 , - 1 , 1 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_lt_u8_true , vec_all_lt , u8x16 -> bool , [0 , 254 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_lt_i16_true , vec_all_lt , i16x8 -> bool , [0 , - 5 , 2 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 42 , 1 , 1 , 1 , 1 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_lt_u16_true , vec_all_lt , u16x8 -> bool , [2 , 254 , 0 , 0 , 0 , 0 , 0 , 0] , [42 , 255 , 1 , 1 , 1 , 1 , 1 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_lt_i32_true , vec_all_lt , i32x4 -> bool , [0 , - 2 , 0 , 0] , [1 , - 1 , 1 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_lt_u32_true , vec_all_lt , u32x4 -> bool , [0 , 254 , 0 , 0] , [1 , 255 , 1 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_lt_i8_false , vec_any_lt , i8x16 -> bool , [1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_lt_u8_false , vec_any_lt , u8x16 -> bool , [42 , 255 , 255 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , [1 , 254 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_lt_i16_false , vec_any_lt , i16x8 -> bool , [2 , 0 , - 1 , 1 , 1 , 1 , 1 , 1] , [1 , - 1 , - 2 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_lt_u16_false , vec_any_lt , u16x8 -> bool , [2 , 42 , 255 , 1 , 1 , 1 , 1 , 1] , [1 , 2 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_lt_i32_false , vec_any_lt , i32x4 -> bool , [2 , 0 , 1 , 1] , [1 , - 1 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_lt_u32_false , vec_any_lt , u32x4 -> bool , [4 , 255 , 4 , 1] , [1 , 2 , 1 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_lt_i8_true , vec_any_lt , i8x16 -> bool , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_lt_u8_true , vec_any_lt , u8x16 -> bool , [0 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_lt_i16_true , vec_any_lt , i16x8 -> bool , [0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_lt_u16_true , vec_any_lt , u16x8 -> bool , [0 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_lt_i32_true , vec_any_lt , i32x4 -> bool , [0 , - 1 , 0 , 1] , [1 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_lt_u32_true , vec_any_lt , u32x4 -> bool , [0 , 255 , 0 , 1] , [1 , 255 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ne_i8_false , vec_all_ne , i8x16 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ne_u8_false , vec_all_ne , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , 255 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ne_i16_false , vec_all_ne , i16x8 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ne_u16_false , vec_all_ne , u16x8 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 255 , 0 , 1 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ne_i32_false , vec_all_ne , i32x4 -> bool , [1 , - 1 , 0 , 0] , [0 , - 1 , 0 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ne_u32_false , vec_all_ne , u32x4 -> bool , [1 , 255 , 0 , 0] , [0 , 255 , 0 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_all_ne_i8_true , vec_all_ne , i8x16 -> bool , [0 , - 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , [1 , 0 , - 1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ne_u8_true , vec_all_ne , u8x16 -> bool , [0 , 254 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ne_i16_true , vec_all_ne , i16x8 -> bool , [2 , - 2 , 0 , 1 , 1 , 1 , 1 , 1] , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ne_u16_true , vec_all_ne , u16x8 -> bool , [0 , 254 , 1 , 1 , 0 , 0 , 1 , 0] , [1 , 255 , 0 , 0 , 1 , 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ne_i32_true , vec_all_ne , i32x4 -> bool , [0 , - 2 , 0 , 0] , [1 , - 1 , 1 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_all_ne_u32_true , vec_all_ne , u32x4 -> bool , [1 , 255 , 0 , 0] , [0 , 254 , 1 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ne_i8_false , vec_any_ne , i8x16 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ne_u8_false , vec_any_ne , u8x16 -> bool , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ne_i16_false , vec_any_ne , i16x8 -> bool , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ne_u16_false , vec_any_ne , u16x8 -> bool , [1 , 255 , 1 , 1 , 1 , 1 , 1 , 0] , [1 , 255 , 1 , 1 , 1 , 1 , 1 , 0] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ne_i32_false , vec_any_ne , i32x4 -> bool , [0 , - 1 , 1 , 1] , [0 , - 1 , 1 , 1] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ne_u32_false , vec_any_ne , u32x4 -> bool , [1 , 2 , 1 , 255] , [1 , 2 , 1 , 255] , false }}
mkitem!{test_vec_2 ! { test_vec_any_ne_i8_true , vec_any_ne , i8x16 -> bool , [1 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [0 , 0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ne_u8_true , vec_any_ne , u8x16 -> bool , [0 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ne_i16_true , vec_any_ne , i16x8 -> bool , [0 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , - 1 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ne_u16_true , vec_any_ne , u16x8 -> bool , [0 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , [1 , 255 , 1 , 0 , 0 , 0 , 0 , 0] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ne_i32_true , vec_any_ne , i32x4 -> bool , [0 , - 1 , 0 , 1] , [1 , - 1 , 0 , 1] , true }}
mkitem!{test_vec_2 ! { test_vec_any_ne_u32_true , vec_any_ne , u32x4 -> bool , [0 , 255 , 0 , 1] , [1 , 255 , 0 , 1] , true }}

macro_rules! test_vec_cmpb_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_cmpb in module {}", module_path!());
    };
}

mkfn!{
    test_vec_cmpb_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_cmpb () { let a : vector_float = transmute (f32x4 :: new (0.1 , 0.5 , 0.6 , 0.9)) ; let b : vector_float = transmute (f32x4 :: new (- 0.1 , 0.5 , - 0.6 , 0.9)) ; let d = i32x4 :: new (- 0b10000000000000000000000000000000 , 0 , - 0b10000000000000000000000000000000 , 0 ,) ; assert_eq ! (d , transmute (vec_cmpb (a , b))) ; }
}

macro_rules! test_vec_ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_ceil in module {}", module_path!());
    };
}

mkfn!{
    test_vec_ceil_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_ceil () { let a : vector_float = transmute (f32x4 :: new (0.1 , 0.5 , 0.6 , 0.9)) ; let d = f32x4 :: new (1.0 , 1.0 , 1.0 , 1.0) ; assert_eq ! (d , transmute (vec_ceil (a))) ; }
}
mkitem!{test_vec_2 ! { test_vec_andc , vec_andc , i32x4 , [0b11001100 , 0b11001100 , 0b11001100 , 0b11001100] , [0b00110011 , 0b11110011 , 0b00001100 , 0b10000000] , [0b11001100 , 0b00001100 , 0b11000000 , 0b01001100] }}
mkitem!{test_vec_2 ! { test_vec_and , vec_and , i32x4 , [0b11001100 , 0b11001100 , 0b11001100 , 0b11001100] , [0b00110011 , 0b11110011 , 0b00001100 , 0b00000000] , [0b00000000 , 0b11000000 , 0b00001100 , 0b00000000] }}
mkitem!{macro_rules ! test_vec_avg { { $ name : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { test_vec_2 ! { $ name , vec_avg , $ ty , [$ ($ a) ,+] , [$ ($ b) ,+] , [$ ($ d) ,+] } } }}
mkitem!{test_vec_avg ! { test_vec_avg_i32x4 , i32x4 , [i32 :: MIN , i32 :: MAX , 1 , - 1] , [- 1 , 1 , 1 , - 1] , [- 1073741824 , 1073741824 , 1 , - 1] }}
mkitem!{test_vec_avg ! { test_vec_avg_u32x4 , u32x4 , [u32 :: MAX , 0 , 1 , 2] , [2 , 1 , 0 , 0] , [2147483649 , 1 , 1 , 1] }}
mkitem!{test_vec_avg ! { test_vec_avg_i16x8 , i16x8 , [i16 :: MIN , i16 :: MAX , 1 , - 1 , 0 , 0 , 0 , 0] , [- 1 , 1 , 1 , - 1 , 0 , 0 , 0 , 0] , [- 16384 , 16384 , 1 , - 1 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_avg ! { test_vec_avg_u16x8 , u16x8 , [u16 :: MAX , 0 , 1 , 2 , 0 , 0 , 0 , 0] , [2 , 1 , 0 , 0 , 0 , 0 , 0 , 0] , [32769 , 1 , 1 , 1 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_avg ! { test_vec_avg_i8x16 , i8x16 , [i8 :: MIN , i8 :: MAX , 1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [- 1 , 1 , 1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [- 64 , 64 , 1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_avg ! { test_vec_avg_u8x16 , u8x16 , [u8 :: MAX , 0 , 1 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [2 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [129 , 1 , 1 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] }}
mkitem!{macro_rules ! test_vec_adds { { $ name : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { test_vec_2 ! { $ name , vec_adds , $ ty , [$ ($ a) ,+] , [$ ($ b) ,+] , [$ ($ d) ,+] } } }}
mkitem!{test_vec_adds ! { test_vec_adds_i32x4 , i32x4 , [i32 :: MIN , i32 :: MAX , 1 , - 1] , [- 1 , 1 , 1 , - 1] , [i32 :: MIN , i32 :: MAX , 2 , - 2] }}
mkitem!{test_vec_adds ! { test_vec_adds_u32x4 , u32x4 , [u32 :: MAX , 0 , 1 , 2] , [2 , 1 , 0 , 0] , [u32 :: MAX , 1 , 1 , 2] }}
mkitem!{test_vec_adds ! { test_vec_adds_i16x8 , i16x8 , [i16 :: MIN , i16 :: MAX , 1 , - 1 , 0 , 0 , 0 , 0] , [- 1 , 1 , 1 , - 1 , 0 , 0 , 0 , 0] , [i16 :: MIN , i16 :: MAX , 2 , - 2 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_adds ! { test_vec_adds_u16x8 , u16x8 , [u16 :: MAX , 0 , 1 , 2 , 0 , 0 , 0 , 0] , [2 , 1 , 0 , 0 , 0 , 0 , 0 , 0] , [u16 :: MAX , 1 , 1 , 2 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_adds ! { test_vec_adds_i8x16 , i8x16 , [i8 :: MIN , i8 :: MAX , 1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [- 1 , 1 , 1 , - 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [i8 :: MIN , i8 :: MAX , 2 , - 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_adds ! { test_vec_adds_u8x16 , u8x16 , [u8 :: MAX , 0 , 1 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [2 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] , [u8 :: MAX , 1 , 1 , 2 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_2 ! { test_vec_addc , vec_addc , u32x4 , [u32 :: MAX , 0 , 0 , 0] , [1 , 1 , 1 , 1] , [1 , 0 , 0 , 0] }}
mkitem!{macro_rules ! test_vec_abs { { $ name : ident , $ ty : ident , $ a : expr , $ d : expr } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a = vec_splats ($ a) ; let a : s_t_l ! ($ ty) = vec_abs (a) ; let d = $ ty :: splat ($ d) ; assert_eq ! (d , transmute (a)) ; } } }}
mkitem!{test_vec_abs ! { test_vec_abs_i8 , i8x16 , - 42i8 , 42i8 }}
mkitem!{test_vec_abs ! { test_vec_abs_i16 , i16x8 , - 42i16 , 42i16 }}
mkitem!{test_vec_abs ! { test_vec_abs_i32 , i32x4 , - 42i32 , 42i32 }}
mkitem!{test_vec_abs ! { test_vec_abs_f32 , f32x4 , - 42f32 , 42f32 }}
mkitem!{macro_rules ! test_vec_abss { { $ name : ident , $ ty : ident , $ a : expr , $ d : expr } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a = vec_splats ($ a) ; let a : s_t_l ! ($ ty) = vec_abss (a) ; let d = $ ty :: splat ($ d) ; assert_eq ! (d , transmute (a)) ; } } }}
mkitem!{test_vec_abss ! { test_vec_abss_i8 , i8x16 , - 127i8 , 127i8 }}
mkitem!{test_vec_abss ! { test_vec_abss_i16 , i16x8 , - 42i16 , 42i16 }}
mkitem!{test_vec_abss ! { test_vec_abss_i32 , i32x4 , - 42i32 , 42i32 }}
mkitem!{macro_rules ! test_vec_splats { { $ name : ident , $ ty : ident , $ a : expr } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : s_t_l ! ($ ty) = vec_splats ($ a) ; let d = $ ty :: splat ($ a) ; assert_eq ! (d , transmute (a)) ; } } }}
mkitem!{test_vec_splats ! { test_vec_splats_u8 , u8x16 , 42u8 }}
mkitem!{test_vec_splats ! { test_vec_splats_u16 , u16x8 , 42u16 }}
mkitem!{test_vec_splats ! { test_vec_splats_u32 , u32x4 , 42u32 }}
mkitem!{test_vec_splats ! { test_vec_splats_i8 , i8x16 , 42i8 }}
mkitem!{test_vec_splats ! { test_vec_splats_i16 , i16x8 , 42i16 }}
mkitem!{test_vec_splats ! { test_vec_splats_i32 , i32x4 , 42i32 }}
mkitem!{test_vec_splats ! { test_vec_splats_f32 , f32x4 , 42f32 }}
mkitem!{macro_rules ! test_vec_splat { { $ name : ident , $ fun : ident , $ ty : ident , $ a : expr , $ b : expr } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a = $ fun ::<$ a > () ; let d = $ ty :: splat ($ b) ; assert_eq ! (d , transmute (a)) ; } } }}
mkitem!{test_vec_splat ! { test_vec_splat_u8 , vec_splat_u8 , u8x16 , - 1 , u8 :: MAX }}
mkitem!{test_vec_splat ! { test_vec_splat_u16 , vec_splat_u16 , u16x8 , - 1 , u16 :: MAX }}
mkitem!{test_vec_splat ! { test_vec_splat_u32 , vec_splat_u32 , u32x4 , - 1 , u32 :: MAX }}
mkitem!{test_vec_splat ! { test_vec_splat_s8 , vec_splat_s8 , i8x16 , - 1 , - 1 }}
mkitem!{test_vec_splat ! { test_vec_splat_s16 , vec_splat_s16 , i16x8 , - 1 , - 1 }}
mkitem!{test_vec_splat ! { test_vec_splat_s32 , vec_splat_s32 , i32x4 , - 1 , - 1 }}
mkitem!{macro_rules ! test_vec_sub { { $ name : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { test_vec_2 ! { $ name , vec_sub , $ ty , [$ ($ a) ,+] , [$ ($ b) ,+] , [$ ($ d) ,+] } } }}
mkitem!{test_vec_sub ! { test_vec_sub_f32x4 , f32x4 , [- 1.0 , 0.0 , 1.0 , 2.0] , [2.0 , 1.0 , - 1.0 , - 2.0] , [- 3.0 , - 1.0 , 2.0 , 4.0] }}
mkitem!{test_vec_sub ! { test_vec_sub_i32x4 , i32x4 , [- 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2] , [- 3 , - 1 , 2 , 4] }}
mkitem!{test_vec_sub ! { test_vec_sub_u32x4 , u32x4 , [0 , 0 , 1 , 2] , [2 , 1 , 0 , 0] , [4294967294 , 4294967295 , 1 , 2] }}
mkitem!{test_vec_sub ! { test_vec_sub_i16x8 , i16x8 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [- 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4] }}
mkitem!{test_vec_sub ! { test_vec_sub_u16x8 , u16x8 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [65534 , 65535 , 1 , 2 , 65534 , 65535 , 1 , 2] }}
mkitem!{test_vec_sub ! { test_vec_sub_i8x16 , i8x16 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [- 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4] }}
mkitem!{test_vec_sub ! { test_vec_sub_u8x16 , u8x16 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [254 , 255 , 1 , 2 , 254 , 255 , 1 , 2 , 254 , 255 , 1 , 2 , 254 , 255 , 1 , 2] }}
mkitem!{macro_rules ! test_vec_subs { { $ name : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { test_vec_2 ! { $ name , vec_subs , $ ty , [$ ($ a) ,+] , [$ ($ b) ,+] , [$ ($ d) ,+] } } }}
mkitem!{test_vec_subs ! { test_vec_subs_i32x4 , i32x4 , [- 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2] , [- 3 , - 1 , 2 , 4] }}
mkitem!{test_vec_subs ! { test_vec_subs_u32x4 , u32x4 , [0 , 0 , 1 , 2] , [2 , 1 , 0 , 0] , [0 , 0 , 1 , 2] }}
mkitem!{test_vec_subs ! { test_vec_subs_i16x8 , i16x8 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [- 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4] }}
mkitem!{test_vec_subs ! { test_vec_subs_u16x8 , u16x8 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] }}
mkitem!{test_vec_subs ! { test_vec_subs_i8x16 , i8x16 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [- 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4 , - 3 , - 1 , 2 , 4] }}
mkitem!{test_vec_subs ! { test_vec_subs_u8x16 , u8x16 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] }}
mkitem!{macro_rules ! test_vec_min { { $ name : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ a) ,+)) ; let b : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ b) ,+)) ; let d = $ ty :: new ($ ($ d) ,+) ; let r : $ ty = transmute (vec_min (a , b)) ; assert_eq ! (d , r) ; } } }}
mkitem!{test_vec_min ! { test_vec_min_i32x4 , i32x4 , [- 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2] , [- 1 , 0 , - 1 , - 2] }}
mkitem!{test_vec_min ! { test_vec_min_u32x4 , u32x4 , [0 , 0 , 1 , 2] , [2 , 1 , 0 , 0] , [0 , 0 , 0 , 0] }}
mkitem!{test_vec_min ! { test_vec_min_i16x8 , i16x8 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [- 1 , 0 , - 1 , - 2 , - 1 , 0 , - 1 , - 2] }}
mkitem!{test_vec_min ! { test_vec_min_u16x8 , u16x8 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] }}
mkitem!{test_vec_min ! { test_vec_min_i8x16 , i8x16 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [- 1 , 0 , - 1 , - 2 , - 1 , 0 , - 1 , - 2 , - 1 , 0 , - 1 , - 2 , - 1 , 0 , - 1 , - 2] }}
mkitem!{test_vec_min ! { test_vec_min_u8x16 , u8x16 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0] }}
mkitem!{macro_rules ! test_vec_max { { $ name : ident , $ ty : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ d : expr) ,+] } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ a) ,+)) ; let b : s_t_l ! ($ ty) = transmute ($ ty :: new ($ ($ b) ,+)) ; let d = $ ty :: new ($ ($ d) ,+) ; let r : $ ty = transmute (vec_max (a , b)) ; assert_eq ! (d , r) ; } } }}
mkitem!{test_vec_max ! { test_vec_max_i32x4 , i32x4 , [- 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2] , [2 , 1 , 1 , 2] }}
mkitem!{test_vec_max ! { test_vec_max_u32x4 , u32x4 , [0 , 0 , 1 , 2] , [2 , 1 , 0 , 0] , [2 , 1 , 1 , 2] }}
mkitem!{test_vec_max ! { test_vec_max_i16x8 , i16x8 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [2 , 1 , 1 , 2 , 2 , 1 , 1 , 2] }}
mkitem!{test_vec_max ! { test_vec_max_u16x8 , u16x8 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [2 , 1 , 1 , 2 , 2 , 1 , 1 , 2] }}
mkitem!{test_vec_max ! { test_vec_max_i8x16 , i8x16 , [- 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2 , - 1 , 0 , 1 , 2] , [2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2 , 2 , 1 , - 1 , - 2] , [2 , 1 , 1 , 2 , 2 , 1 , 1 , 2 , 2 , 1 , 1 , 2 , 2 , 1 , 1 , 2] }}
mkitem!{test_vec_max ! { test_vec_max_u8x16 , u8x16 , [0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2 , 0 , 0 , 1 , 2] , [2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0 , 2 , 1 , 0 , 0] , [2 , 1 , 1 , 2 , 2 , 1 , 1 , 2 , 2 , 1 , 1 , 2 , 2 , 1 , 1 , 2] }}
mkitem!{macro_rules ! test_vec_perm { { $ name : ident , $ shorttype : ident , $ longtype : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ c : expr) ,+] , [$ ($ d : expr) ,+] } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : $ longtype = transmute ($ shorttype :: new ($ ($ a) ,+)) ; let b : $ longtype = transmute ($ shorttype :: new ($ ($ b) ,+)) ; let c : vector_unsigned_char = transmute (u8x16 :: new ($ ($ c) ,+)) ; let d = $ shorttype :: new ($ ($ d) ,+) ; let r : $ shorttype = transmute (vec_perm (a , b , c)) ; assert_eq ! (d , r) ; } } }}
mkitem!{test_vec_perm ! { test_vec_perm_u8x16 , u8x16 , vector_unsigned_char , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15] , [100 , 101 , 102 , 103 , 104 , 105 , 106 , 107 , 108 , 109 , 110 , 111 , 112 , 113 , 114 , 115] , [0x00 , 0x01 , 0x10 , 0x11 , 0x02 , 0x03 , 0x12 , 0x13 , 0x04 , 0x05 , 0x14 , 0x15 , 0x06 , 0x07 , 0x16 , 0x17] , [0 , 1 , 100 , 101 , 2 , 3 , 102 , 103 , 4 , 5 , 104 , 105 , 6 , 7 , 106 , 107] }}
mkitem!{test_vec_perm ! { test_vec_perm_i8x16 , i8x16 , vector_signed_char , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15] , [100 , 101 , 102 , 103 , 104 , 105 , 106 , 107 , 108 , 109 , 110 , 111 , 112 , 113 , 114 , 115] , [0x00 , 0x01 , 0x10 , 0x11 , 0x02 , 0x03 , 0x12 , 0x13 , 0x04 , 0x05 , 0x14 , 0x15 , 0x06 , 0x07 , 0x16 , 0x17] , [0 , 1 , 100 , 101 , 2 , 3 , 102 , 103 , 4 , 5 , 104 , 105 , 6 , 7 , 106 , 107] }}
mkitem!{test_vec_perm ! { test_vec_perm_m8x16 , m8x16 , vector_bool_char , [false , false , false , false , false , false , false , false , false , false , false , false , false , false , false , false] , [true , true , true , true , true , true , true , true , true , true , true , true , true , true , true , true] , [0x00 , 0x01 , 0x10 , 0x11 , 0x02 , 0x03 , 0x12 , 0x13 , 0x04 , 0x05 , 0x14 , 0x15 , 0x06 , 0x07 , 0x16 , 0x17] , [false , false , true , true , false , false , true , true , false , false , true , true , false , false , true , true] }}
mkitem!{test_vec_perm ! { test_vec_perm_u16x8 , u16x8 , vector_unsigned_short , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [10 , 11 , 12 , 13 , 14 , 15 , 16 , 17] , [0x00 , 0x01 , 0x10 , 0x11 , 0x02 , 0x03 , 0x12 , 0x13 , 0x04 , 0x05 , 0x14 , 0x15 , 0x06 , 0x07 , 0x16 , 0x17] , [0 , 10 , 1 , 11 , 2 , 12 , 3 , 13] }}
mkitem!{test_vec_perm ! { test_vec_perm_i16x8 , i16x8 , vector_signed_short , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [10 , 11 , 12 , 13 , 14 , 15 , 16 , 17] , [0x00 , 0x01 , 0x10 , 0x11 , 0x02 , 0x03 , 0x12 , 0x13 , 0x04 , 0x05 , 0x14 , 0x15 , 0x06 , 0x07 , 0x16 , 0x17] , [0 , 10 , 1 , 11 , 2 , 12 , 3 , 13] }}
mkitem!{test_vec_perm ! { test_vec_perm_m16x8 , m16x8 , vector_bool_short , [false , false , false , false , false , false , false , false] , [true , true , true , true , true , true , true , true] , [0x00 , 0x01 , 0x10 , 0x11 , 0x02 , 0x03 , 0x12 , 0x13 , 0x04 , 0x05 , 0x14 , 0x15 , 0x06 , 0x07 , 0x16 , 0x17] , [false , true , false , true , false , true , false , true] }}
mkitem!{test_vec_perm ! { test_vec_perm_u32x4 , u32x4 , vector_unsigned_int , [0 , 1 , 2 , 3] , [10 , 11 , 12 , 13] , [0x00 , 0x01 , 0x02 , 0x03 , 0x10 , 0x11 , 0x12 , 0x13 , 0x04 , 0x05 , 0x06 , 0x07 , 0x14 , 0x15 , 0x16 , 0x17] , [0 , 10 , 1 , 11] }}
mkitem!{test_vec_perm ! { test_vec_perm_i32x4 , i32x4 , vector_signed_int , [0 , 1 , 2 , 3] , [10 , 11 , 12 , 13] , [0x00 , 0x01 , 0x02 , 0x03 , 0x10 , 0x11 , 0x12 , 0x13 , 0x04 , 0x05 , 0x06 , 0x07 , 0x14 , 0x15 , 0x16 , 0x17] , [0 , 10 , 1 , 11] }}
mkitem!{test_vec_perm ! { test_vec_perm_m32x4 , m32x4 , vector_bool_int , [false , false , false , false] , [true , true , true , true] , [0x00 , 0x01 , 0x02 , 0x03 , 0x10 , 0x11 , 0x12 , 0x13 , 0x04 , 0x05 , 0x06 , 0x07 , 0x14 , 0x15 , 0x16 , 0x17] , [false , true , false , true] }}
mkitem!{test_vec_perm ! { test_vec_perm_f32x4 , f32x4 , vector_float , [0.0 , 1.0 , 2.0 , 3.0] , [1.0 , 1.1 , 1.2 , 1.3] , [0x00 , 0x01 , 0x02 , 0x03 , 0x10 , 0x11 , 0x12 , 0x13 , 0x04 , 0x05 , 0x06 , 0x07 , 0x14 , 0x15 , 0x16 , 0x17] , [0.0 , 1.0 , 1.0 , 1.1] }}

macro_rules! test_vec_madds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_madds in module {}", module_path!());
    };
}

mkfn!{
    test_vec_madds_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_madds () { let a : vector_signed_short = transmute (i16x8 :: new (0 * 256 , 1 * 256 , 2 * 256 , 3 * 256 , 4 * 256 , 5 * 256 , 6 * 256 , 7 * 256 ,)) ; let b : vector_signed_short = transmute (i16x8 :: new (256 , 256 , 256 , 256 , 256 , 256 , 256 , 256)) ; let c : vector_signed_short = transmute (i16x8 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let d = i16x8 :: new (0 , 3 , 6 , 9 , 12 , 15 , 18 , 21) ; assert_eq ! (d , transmute (vec_madds (a , b , c))) ; }
}

macro_rules! test_vec_madd_float_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_madd_float in module {}", module_path!());
    };
}

mkfn!{
    test_vec_madd_float_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_madd_float () { let a : vector_float = transmute (f32x4 :: new (0.1 , 0.2 , 0.3 , 0.4)) ; let b : vector_float = transmute (f32x4 :: new (0.1 , 0.2 , 0.3 , 0.4)) ; let c : vector_float = transmute (f32x4 :: new (0.1 , 0.2 , 0.3 , 0.4)) ; let d = f32x4 :: new (0.1 * 0.1 + 0.1 , 0.2 * 0.2 + 0.2 , 0.3 * 0.3 + 0.3 , 0.4 * 0.4 + 0.4 ,) ; assert_eq ! (d , transmute (vec_madd (a , b , c))) ; }
}

macro_rules! test_vec_nmsub_float_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_nmsub_float in module {}", module_path!());
    };
}

mkfn!{
    test_vec_nmsub_float_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_nmsub_float () { let a : vector_float = transmute (f32x4 :: new (0.1 , 0.2 , 0.3 , 0.4)) ; let b : vector_float = transmute (f32x4 :: new (0.1 , 0.2 , 0.3 , 0.4)) ; let c : vector_float = transmute (f32x4 :: new (0.1 , 0.2 , 0.3 , 0.4)) ; let d = f32x4 :: new (- (0.1 * 0.1 - 0.1) , - (0.2 * 0.2 - 0.2) , - (0.3 * 0.3 - 0.3) , - (0.4 * 0.4 - 0.4) ,) ; assert_eq ! (d , transmute (vec_nmsub (a , b , c))) ; }
}

macro_rules! test_vec_mradds_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mradds in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mradds_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mradds () { let a : vector_signed_short = transmute (i16x8 :: new (0 * 256 , 1 * 256 , 2 * 256 , 3 * 256 , 4 * 256 , 5 * 256 , 6 * 256 , 7 * 256 ,)) ; let b : vector_signed_short = transmute (i16x8 :: new (256 , 256 , 256 , 256 , 256 , 256 , 256 , 256)) ; let c : vector_signed_short = transmute (i16x8 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , i16 :: MAX - 1)) ; let d = i16x8 :: new (0 , 3 , 6 , 9 , 12 , 15 , 18 , i16 :: MAX) ; assert_eq ! (d , transmute (vec_mradds (a , b , c))) ; }
}
mkitem!{macro_rules ! test_vec_mladd { { $ name : ident , $ sa : ident , $ la : ident , $ sbc : ident , $ lbc : ident , $ sd : ident , [$ ($ a : expr) ,+] , [$ ($ b : expr) ,+] , [$ ($ c : expr) ,+] , [$ ($ d : expr) ,+] } => { # [simd_test (enable = "altivec")] unsafe fn $ name () { let a : $ la = transmute ($ sa :: new ($ ($ a) ,+)) ; let b : $ lbc = transmute ($ sbc :: new ($ ($ b) ,+)) ; let c = transmute ($ sbc :: new ($ ($ c) ,+)) ; let d = $ sd :: new ($ ($ d) ,+) ; assert_eq ! (d , transmute (vec_mladd (a , b , c))) ; } } }}
mkitem!{test_vec_mladd ! { test_vec_mladd_u16x8_u16x8 , u16x8 , vector_unsigned_short , u16x8 , vector_unsigned_short , u16x8 , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 2 , 6 , 12 , 20 , 30 , 42 , 56] }}
mkitem!{test_vec_mladd ! { test_vec_mladd_u16x8_i16x8 , u16x8 , vector_unsigned_short , i16x8 , vector_unsigned_short , i16x8 , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 2 , 6 , 12 , 20 , 30 , 42 , 56] }}
mkitem!{test_vec_mladd ! { test_vec_mladd_i16x8_u16x8 , i16x8 , vector_signed_short , u16x8 , vector_unsigned_short , i16x8 , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 2 , 6 , 12 , 20 , 30 , 42 , 56] }}
mkitem!{test_vec_mladd ! { test_vec_mladd_i16x8_i16x8 , i16x8 , vector_signed_short , i16x8 , vector_unsigned_short , i16x8 , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] , [0 , 2 , 6 , 12 , 20 , 30 , 42 , 56] }}

macro_rules! test_vec_msum_unsigned_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_msum_unsigned_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_msum_unsigned_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_msum_unsigned_char () { let a : vector_unsigned_char = transmute (u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let b : vector_unsigned_char = transmute (u8x16 :: new (255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 , 255 ,)) ; let c : vector_unsigned_int = transmute (u32x4 :: new (0 , 1 , 2 , 3)) ; let d = u32x4 :: new ((0 + 1 + 2 + 3) * 255 + 0 , (4 + 5 + 6 + 7) * 255 + 1 , (0 + 1 + 2 + 3) * 255 + 2 , (4 + 5 + 6 + 7) * 255 + 3 ,) ; assert_eq ! (d , transmute (vec_msum (a , b , c))) ; }
}

macro_rules! test_vec_msum_signed_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_msum_signed_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_msum_signed_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_msum_signed_char () { let a : vector_signed_char = transmute (i8x16 :: new (0 , - 1 , 2 , - 3 , 1 , - 1 , 1 , - 1 , 0 , 1 , 2 , 3 , 4 , - 5 , - 6 , - 7 ,)) ; let b : vector_unsigned_char = transmute (i8x16 :: new (1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1 , 1)) ; let c : vector_signed_int = transmute (u32x4 :: new (0 , 1 , 2 , 3)) ; let d = i32x4 :: new ((0 - 1 + 2 - 3) + 0 , (0) + 1 , (0 + 1 + 2 + 3) + 2 , (4 - 5 - 6 - 7) + 3 ,) ; assert_eq ! (d , transmute (vec_msum (a , b , c))) ; }
}

macro_rules! test_vec_msum_unsigned_short_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_msum_unsigned_short in module {}", module_path!());
    };
}

mkfn!{
    test_vec_msum_unsigned_short_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_msum_unsigned_short () { let a : vector_unsigned_short = transmute (u16x8 :: new (0 * 256 , 1 * 256 , 2 * 256 , 3 * 256 , 4 * 256 , 5 * 256 , 6 * 256 , 7 * 256 ,)) ; let b : vector_unsigned_short = transmute (u16x8 :: new (256 , 256 , 256 , 256 , 256 , 256 , 256 , 256)) ; let c : vector_unsigned_int = transmute (u32x4 :: new (0 , 1 , 2 , 3)) ; let d = u32x4 :: new ((0 + 1) * 256 * 256 + 0 , (2 + 3) * 256 * 256 + 1 , (4 + 5) * 256 * 256 + 2 , (6 + 7) * 256 * 256 + 3 ,) ; assert_eq ! (d , transmute (vec_msum (a , b , c))) ; }
}

macro_rules! test_vec_msum_signed_short_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_msum_signed_short in module {}", module_path!());
    };
}

mkfn!{
    test_vec_msum_signed_short_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_msum_signed_short () { let a : vector_signed_short = transmute (i16x8 :: new (0 * 256 , - 1 * 256 , 2 * 256 , - 3 * 256 , 4 * 256 , - 5 * 256 , 6 * 256 , - 7 * 256 ,)) ; let b : vector_signed_short = transmute (i16x8 :: new (256 , 256 , 256 , 256 , 256 , 256 , 256 , 256)) ; let c : vector_signed_int = transmute (i32x4 :: new (0 , 1 , 2 , 3)) ; let d = i32x4 :: new ((0 - 1) * 256 * 256 + 0 , (2 - 3) * 256 * 256 + 1 , (4 - 5) * 256 * 256 + 2 , (6 - 7) * 256 * 256 + 3 ,) ; assert_eq ! (d , transmute (vec_msum (a , b , c))) ; }
}

macro_rules! test_vec_msums_unsigned_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_msums_unsigned in module {}", module_path!());
    };
}

mkfn!{
    test_vec_msums_unsigned_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_msums_unsigned () { let a : vector_unsigned_short = transmute (u16x8 :: new (0 * 256 , 1 * 256 , 2 * 256 , 3 * 256 , 4 * 256 , 5 * 256 , 6 * 256 , 7 * 256 ,)) ; let b : vector_unsigned_short = transmute (u16x8 :: new (256 , 256 , 256 , 256 , 256 , 256 , 256 , 256)) ; let c : vector_unsigned_int = transmute (u32x4 :: new (0 , 1 , 2 , 3)) ; let d = u32x4 :: new ((0 + 1) * 256 * 256 + 0 , (2 + 3) * 256 * 256 + 1 , (4 + 5) * 256 * 256 + 2 , (6 + 7) * 256 * 256 + 3 ,) ; assert_eq ! (d , transmute (vec_msums (a , b , c))) ; }
}

macro_rules! test_vec_msums_signed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_msums_signed in module {}", module_path!());
    };
}

mkfn!{
    test_vec_msums_signed_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_msums_signed () { let a : vector_signed_short = transmute (i16x8 :: new (0 * 256 , - 1 * 256 , 2 * 256 , - 3 * 256 , 4 * 256 , - 5 * 256 , 6 * 256 , - 7 * 256 ,)) ; let b : vector_signed_short = transmute (i16x8 :: new (256 , 256 , 256 , 256 , 256 , 256 , 256 , 256)) ; let c : vector_signed_int = transmute (i32x4 :: new (0 , 1 , 2 , 3)) ; let d = i32x4 :: new ((0 - 1) * 256 * 256 + 0 , (2 - 3) * 256 * 256 + 1 , (4 - 5) * 256 * 256 + 2 , (6 - 7) * 256 * 256 + 3 ,) ; assert_eq ! (d , transmute (vec_msums (a , b , c))) ; }
}

macro_rules! test_vec_sum2s_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_sum2s in module {}", module_path!());
    };
}

mkfn!{
    test_vec_sum2s_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_sum2s () { let a : vector_signed_int = transmute (i32x4 :: new (0 , 1 , 2 , 3)) ; let b : vector_signed_int = transmute (i32x4 :: new (0 , 1 , 2 , 3)) ; let d = i32x4 :: new (0 , 0 + 1 + 1 , 0 , 2 + 3 + 3) ; assert_eq ! (d , transmute (vec_sum2s (a , b))) ; }
}

macro_rules! test_vec_sum4s_unsigned_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_sum4s_unsigned_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_sum4s_unsigned_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_sum4s_unsigned_char () { let a : vector_unsigned_char = transmute (u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let b : vector_unsigned_int = transmute (u32x4 :: new (0 , 1 , 2 , 3)) ; let d = u32x4 :: new (0 + 1 + 2 + 3 + 0 , 4 + 5 + 6 + 7 + 1 , 0 + 1 + 2 + 3 + 2 , 4 + 5 + 6 + 7 + 3 ,) ; assert_eq ! (d , transmute (vec_sum4s (a , b))) ; }
}

macro_rules! test_vec_sum4s_signed_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_sum4s_signed_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_sum4s_signed_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_sum4s_signed_char () { let a : vector_signed_char = transmute (i8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let b : vector_signed_int = transmute (i32x4 :: new (0 , 1 , 2 , 3)) ; let d = i32x4 :: new (0 + 1 + 2 + 3 + 0 , 4 + 5 + 6 + 7 + 1 , 0 + 1 + 2 + 3 + 2 , 4 + 5 + 6 + 7 + 3 ,) ; assert_eq ! (d , transmute (vec_sum4s (a , b))) ; }
}

macro_rules! test_vec_sum4s_signed_short_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_sum4s_signed_short in module {}", module_path!());
    };
}

mkfn!{
    test_vec_sum4s_signed_short_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_sum4s_signed_short () { let a : vector_signed_short = transmute (i16x8 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let b : vector_signed_int = transmute (i32x4 :: new (0 , 1 , 2 , 3)) ; let d = i32x4 :: new (0 + 1 + 0 , 2 + 3 + 1 , 4 + 5 + 2 , 6 + 7 + 3) ; assert_eq ! (d , transmute (vec_sum4s (a , b))) ; }
}

macro_rules! test_vec_mule_unsigned_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mule_unsigned_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mule_unsigned_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mule_unsigned_char () { let a : vector_unsigned_char = transmute (u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let d = u16x8 :: new (0 * 0 , 2 * 2 , 4 * 4 , 6 * 6 , 0 * 0 , 2 * 2 , 4 * 4 , 6 * 6) ; assert_eq ! (d , transmute (vec_mule (a , a))) ; }
}

macro_rules! test_vec_mule_signed_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mule_signed_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mule_signed_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mule_signed_char () { let a : vector_signed_char = transmute (i8x16 :: new (0 , 1 , - 2 , 3 , - 4 , 5 , - 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 ,)) ; let d = i16x8 :: new (0 * 0 , 2 * 2 , 4 * 4 , 6 * 6 , 0 * 0 , 2 * 2 , 4 * 4 , 6 * 6) ; assert_eq ! (d , transmute (vec_mule (a , a))) ; }
}

macro_rules! test_vec_mule_unsigned_short_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mule_unsigned_short in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mule_unsigned_short_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mule_unsigned_short () { let a : vector_unsigned_short = transmute (u16x8 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let d = u32x4 :: new (0 * 0 , 2 * 2 , 4 * 4 , 6 * 6) ; assert_eq ! (d , transmute (vec_mule (a , a))) ; }
}

macro_rules! test_vec_mule_signed_short_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mule_signed_short in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mule_signed_short_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mule_signed_short () { let a : vector_signed_short = transmute (i16x8 :: new (0 , 1 , - 2 , 3 , - 4 , 5 , - 6 , 7)) ; let d = i32x4 :: new (0 * 0 , 2 * 2 , 4 * 4 , 6 * 6) ; assert_eq ! (d , transmute (vec_mule (a , a))) ; }
}

macro_rules! test_vec_mulo_unsigned_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mulo_unsigned_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mulo_unsigned_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mulo_unsigned_char () { let a : vector_unsigned_char = transmute (u8x16 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let d = u16x8 :: new (1 * 1 , 3 * 3 , 5 * 5 , 7 * 7 , 1 * 1 , 3 * 3 , 5 * 5 , 7 * 7) ; assert_eq ! (d , transmute (vec_mulo (a , a))) ; }
}

macro_rules! test_vec_mulo_signed_char_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mulo_signed_char in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mulo_signed_char_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mulo_signed_char () { let a : vector_signed_char = transmute (i8x16 :: new (0 , 1 , - 2 , 3 , - 4 , 5 , - 6 , 7 , 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 ,)) ; let d = i16x8 :: new (1 * 1 , 3 * 3 , 5 * 5 , 7 * 7 , 1 * 1 , 3 * 3 , 5 * 5 , 7 * 7) ; assert_eq ! (d , transmute (vec_mulo (a , a))) ; }
}

macro_rules! test_vec_mulo_unsigned_short_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mulo_unsigned_short in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mulo_unsigned_short_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mulo_unsigned_short () { let a : vector_unsigned_short = transmute (u16x8 :: new (0 , 1 , 2 , 3 , 4 , 5 , 6 , 7)) ; let d = u32x4 :: new (1 * 1 , 3 * 3 , 5 * 5 , 7 * 7) ; assert_eq ! (d , transmute (vec_mulo (a , a))) ; }
}

macro_rules! test_vec_mulo_signed_short_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_mulo_signed_short in module {}", module_path!());
    };
}

mkfn!{
    test_vec_mulo_signed_short_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_mulo_signed_short () { let a : vector_signed_short = transmute (i16x8 :: new (0 , 1 , - 2 , 3 , - 4 , 5 , - 6 , 7)) ; let d = i32x4 :: new (1 * 1 , 3 * 3 , 5 * 5 , 7 * 7) ; assert_eq ! (d , transmute (vec_mulo (a , a))) ; }
}

macro_rules! vec_add_i32x4_i32x4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_add_i32x4_i32x4 in module {}", module_path!());
    };
}

mkfn!{
    vec_add_i32x4_i32x4_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn vec_add_i32x4_i32x4 () { let x = i32x4 :: new (1 , 2 , 3 , 4) ; let y = i32x4 :: new (4 , 3 , 2 , 1) ; let x : vector_signed_int = transmute (x) ; let y : vector_signed_int = transmute (y) ; let z = vec_add (x , y) ; assert_eq ! (i32x4 :: splat (5) , transmute (z)) ; }
}

macro_rules! vec_ctf_u32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ctf_u32 in module {}", module_path!());
    };
}

mkfn!{
    vec_ctf_u32_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn vec_ctf_u32 () { let v : vector_unsigned_int = transmute (u32x4 :: new (u32 :: MIN , u32 :: MAX , u32 :: MAX , 42)) ; let v2 = vec_ctf :: < 1 , _ > (v) ; let r2 : vector_float = transmute (f32x4 :: new (0.0 , 2147483600.0 , 2147483600.0 , 21.0)) ; let v4 = vec_ctf :: < 2 , _ > (v) ; let r4 : vector_float = transmute (f32x4 :: new (0.0 , 1073741800.0 , 1073741800.0 , 10.5)) ; let v8 = vec_ctf :: < 3 , _ > (v) ; let r8 : vector_float = transmute (f32x4 :: new (0.0 , 536870900.0 , 536870900.0 , 5.25)) ; let check = | a , b | { let r = transmute (vec_cmple (vec_abs (vec_sub (a , b)) , vec_splats (f32 :: EPSILON))) ; let e = m32x4 :: new (true , true , true , true) ; assert_eq ! (e , r) ; } ; check (v2 , r2) ; check (v4 , r4) ; check (v8 , r8) ; }
}

macro_rules! test_vec_ctu_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_ctu in module {}", module_path!());
    };
}

mkfn!{
    test_vec_ctu_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_ctu () { let v = u32x4 :: new (u32 :: MIN , u32 :: MAX , u32 :: MAX , 42) ; let v2 : u32x4 = transmute (vec_ctu :: < 1 > (transmute (f32x4 :: new (0.0 , 2147483600.0 , 2147483600.0 , 21.0 ,)))) ; let v4 : u32x4 = transmute (vec_ctu :: < 2 > (transmute (f32x4 :: new (0.0 , 1073741800.0 , 1073741800.0 , 10.5 ,)))) ; let v8 : u32x4 = transmute (vec_ctu :: < 3 > (transmute (f32x4 :: new (0.0 , 536870900.0 , 536870900.0 , 5.25 ,)))) ; assert_eq ! (v2 , v) ; assert_eq ! (v4 , v) ; assert_eq ! (v8 , v) ; }
}

macro_rules! vec_ctf_i32_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_ctf_i32 in module {}", module_path!());
    };
}

mkfn!{
    vec_ctf_i32_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn vec_ctf_i32 () { let v : vector_signed_int = transmute (i32x4 :: new (i32 :: MIN , i32 :: MAX , i32 :: MAX - 42 , 42)) ; let v2 = vec_ctf :: < 1 , _ > (v) ; let r2 : vector_float = transmute (f32x4 :: new (- 1073741800.0 , 1073741800.0 , 1073741800.0 , 21.0)) ; let v4 = vec_ctf :: < 2 , _ > (v) ; let r4 : vector_float = transmute (f32x4 :: new (- 536870900.0 , 536870900.0 , 536870900.0 , 10.5)) ; let v8 = vec_ctf :: < 3 , _ > (v) ; let r8 : vector_float = transmute (f32x4 :: new (- 268435460.0 , 268435460.0 , 268435460.0 , 5.25)) ; let check = | a , b | { let r = transmute (vec_cmple (vec_abs (vec_sub (a , b)) , vec_splats (f32 :: EPSILON))) ; println ! ("{:?} {:?}" , a , b) ; let e = m32x4 :: new (true , true , true , true) ; assert_eq ! (e , r) ; } ; check (v2 , r2) ; check (v4 , r4) ; check (v8 , r8) ; }
}

macro_rules! test_vec_cts_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_vec_cts in module {}", module_path!());
    };
}

mkfn!{
    test_vec_cts_introspect!();
    # [simd_test (enable = "altivec")] unsafe fn test_vec_cts () { let v = i32x4 :: new (i32 :: MIN , i32 :: MAX , i32 :: MAX , 42) ; let v2 : i32x4 = transmute (vec_cts :: < 1 > (transmute (f32x4 :: new (- 1073741800.0 , 1073741800.0 , 1073741800.0 , 21.0 ,)))) ; let v4 : i32x4 = transmute (vec_cts :: < 2 > (transmute (f32x4 :: new (- 536870900.0 , 536870900.0 , 536870900.0 , 10.5 ,)))) ; let v8 : i32x4 = transmute (vec_cts :: < 3 > (transmute (f32x4 :: new (- 268435460.0 , 268435460.0 , 268435460.0 , 5.25 ,)))) ; assert_eq ! (v2 , v) ; assert_eq ! (v4 , v) ; assert_eq ! (v8 , v) ; }
}
mkitem!{test_vec_2 ! { test_vec_rl , vec_rl , u32x4 , [0x12345678 , 0x9ABCDEF0 , 0x0F0F0F0F , 0x12345678] , [4 , 8 , 12 , 68] , [0x23456781 , 0xBCDEF09A , 0xF0F0F0F0 , 0x23456781] }} 
            }}