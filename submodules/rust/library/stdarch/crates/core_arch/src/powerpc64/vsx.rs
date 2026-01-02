mkuse!{use crate :: core_arch :: powerpc :: macros :: * ;}
mkuse!{use crate :: core_arch :: powerpc :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}
mkuse!{use crate :: mem :: transmute ;}
mkitem!{# [allow (improper_ctypes)] unsafe extern "C" { # [link_name = "llvm.ppc.vsx.lxvl"] fn lxvl (a : * const u8 , l : usize) -> vector_signed_int ; # [link_name = "llvm.ppc.vsx.stxvl"] fn stxvl (v : vector_signed_int , a : * mut u8 , l : usize) ; }}
mkmod!{sealed, { 
                getname!(sealed);
                getsrc!(sealed);
                getpath!(sealed);
                get_deps!(sealed);
                get_crates!(sealed);
                mkinclude!(sealed);
                mkuse!{use super :: * ;}

macro_rules! vec_lxvl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_lxvl in module {}", module_path!());
    };
}

mkfn!{
    vec_lxvl_introspect!();
    # [inline] # [target_feature (enable = "power9-vector")] # [cfg_attr (test , assert_instr (lxvl))] unsafe fn vec_lxvl (p : * const u8 , l : usize) -> vector_signed_int { lxvl (p , l << 56) }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorXloads { type Result ; unsafe fn vec_xl_len (self , l : usize) -> Self :: Result ; }}}
mkitem!{macro_rules ! impl_vsx_loads { ($ ty : ident) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorXloads for * const $ ty { type Result = t_t_l ! ($ ty) ; # [inline] # [target_feature (enable = "power9-vector")] unsafe fn vec_xl_len (self , l : usize) -> Self :: Result { transmute (vec_lxvl (self as * const u8 , l)) } } } ; }}
mkitem!{impl_vsx_loads ! { i8 }}
mkitem!{impl_vsx_loads ! { u8 }}
mkitem!{impl_vsx_loads ! { i16 }}
mkitem!{impl_vsx_loads ! { u16 }}
mkitem!{impl_vsx_loads ! { i32 }}
mkitem!{impl_vsx_loads ! { u32 }}
mkitem!{impl_vsx_loads ! { f32 }}

macro_rules! vec_stxvl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_stxvl in module {}", module_path!());
    };
}

mkfn!{
    vec_stxvl_introspect!();
    # [inline] # [target_feature (enable = "power9-vector")] # [cfg_attr (test , assert_instr (stxvl))] unsafe fn vec_stxvl (v : vector_signed_int , a : * mut u8 , l : usize) { stxvl (v , a , l << 56) ; }
}
mkitem!{mktrait!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub trait VectorXstores { type Out ; unsafe fn vec_xst_len (self , p : Self :: Out , l : usize) ; }}}
mkitem!{macro_rules ! impl_stores { ($ ty : ident) => { # [unstable (feature = "stdarch_powerpc" , issue = "111145")] impl VectorXstores for t_t_l ! ($ ty) { type Out = * mut $ ty ; # [inline] # [target_feature (enable = "power9-vector")] unsafe fn vec_xst_len (self , a : Self :: Out , l : usize) { stxvl (transmute (self) , a as * mut u8 , l) } } } ; }}
mkitem!{impl_stores ! { i8 }}
mkitem!{impl_stores ! { u8 }}
mkitem!{impl_stores ! { i16 }}
mkitem!{impl_stores ! { u16 }}
mkitem!{impl_stores ! { i32 }}
mkitem!{impl_stores ! { u32 }}
mkitem!{impl_stores ! { f32 }} 
            }}

macro_rules! vec_xl_len_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_xl_len in module {}", module_path!());
    };
}

mkfn!{
    vec_xl_len_introspect!();
    # [doc = " Vector Load with Length"] # [doc = ""] # [doc = " ## Purpose"] # [doc = " Loads a vector of a specified byte length."] # [doc = ""] # [doc = " ## Result value"] # [doc = " Loads the number of bytes specified by b from the address specified in a."] # [doc = " Initializes elements in order from the byte stream (as defined by the endianness of the"] # [doc = " target). Any bytes of elements that cannot be initialized from the number of loaded bytes have"] # [doc = " a zero value."] # [doc = ""] # [doc = " Between 0 and 16 bytes, inclusive, will be loaded. The length is specified by the"] # [doc = " least-significant byte of b, as min (b mod 256, 16). The behavior is undefined if the length"] # [doc = " argument is outside of the range 0–255, or if it is not a multiple of the vector element size."] # [doc = ""] # [doc = " ## Notes"] # [doc = " vec_xl_len should not be used to load from cache-inhibited memory."] # [inline] # [target_feature (enable = "power9-vector")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_xl_len < T > (p : T , len : usize) -> < T as sealed :: VectorXloads > :: Result where T : sealed :: VectorXloads , { p . vec_xl_len (len) }
}

macro_rules! vec_xst_len_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vec_xst_len in module {}", module_path!());
    };
}

mkfn!{
    vec_xst_len_introspect!();
    # [doc = " Vector Store with Length"] # [doc = ""] # [doc = " ## Purpose"] # [doc = ""] # [doc = " Stores a vector of a specified byte length."] # [doc = ""] # [doc = " ## Operation"] # [doc = ""] # [doc = " Stores the number of bytes specified by c of the vector a to the address specified"] # [doc = " in b. The bytes are obtained starting from the lowest-numbered byte of the lowest-numbered"] # [doc = " element (as defined by the endianness of the target). All bytes of an element are accessed"] # [doc = " before proceeding to the next higher element."] # [doc = ""] # [doc = " Between 0 and 16 bytes, inclusive, will be stored. The length is specified by the"] # [doc = " least-significant byte of c, as min (c mod 256, 16). The behavior is undefined if the length"] # [doc = " argument is outside of the range 0–255, or if it is not a multiple of the vector element size."] # [doc = ""] # [doc = " ## Notes"] # [doc = " vec_xst_len should not be used to store to cache-inhibited memory."] # [inline] # [target_feature (enable = "power9-vector")] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn vec_xst_len < T > (v : T , a : < T as sealed :: VectorXstores > :: Out , l : usize) where T : sealed :: VectorXstores , { v . vec_xst_len (a , l) }
}