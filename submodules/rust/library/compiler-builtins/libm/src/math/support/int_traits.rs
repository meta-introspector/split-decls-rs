mkuse!{use std :: sync :: Mutex ;}
mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: sync :: LazyLock ;}
mkitem!{static USE_MATRIX : LazyLock < Mutex < HashMap < String , Vec < String > > > > = LazyLock :: new (| | Mutex :: new (HashMap :: new ())) ;}

macro_rules! get_use_matrix_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_use_matrix in module {}", module_path!());
    };
}

mkfn!{
    get_use_matrix_introspect!();
    pub fn get_use_matrix () -> HashMap < String , Vec < String > > { USE_MATRIX . lock () . unwrap () . clone () }
}
mkitem!{macro_rules ! emit_message { ($ ($ arg : tt) *) => { { use std :: fs :: OpenOptions ; use std :: io :: Write ; let message = format ! ($ ($ arg) *) ; if let Ok (mut file) = OpenOptions :: new () . create (true) . append (true) . open ("macro_report.txt") { let _ = writeln ! (file , "{}" , message) ; } } } ; }}
mkitem!{macro_rules ! mkfn { ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: trait_bounds_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn catch_fatal_errors < F : FnOnce () -> R , R > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: catch_fatal_errors_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: catch_fatal_errors_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn parse_crate_attrs < $ lifetime : lifetime > ($ ($ param : tt) *) -> PResult < $ lifetime2 : lifetime , ast :: AttrVec > $ body : block) => { $ (#[$ attr]) * fn parse_crate_attrs < $ lifetime > ($ ($ param) *) -> PResult < $ lifetime2 , ast :: AttrVec > { $ introspect ; emit_message ! ("🚀 MARKER: parse_crate_attrs_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: parse_crate_attrs_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn init_logger_with_additional_layer < F , T > ($ ($ param : tt) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , $ body : block) => { $ (#[$ attr]) * fn init_logger_with_additional_layer < F , T > ($ ($ param) *) where F : FnOnce () -> T , T : rustc_log :: BuildSubscriberRet , { $ introspect ; emit_message ! ("🚀 MARKER: init_logger_specific") ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: init_logger_specific") ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < F , T > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where F : FnOnce ($ ($ fnonce_args : tt) *) $ ($ where_rest : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < F , T > ($ ($ param) *) $ (-> $ ret) ? where F : FnOnce ($ ($ fnonce_args) *) $ ($ where_rest) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where_fnonce - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? where $ ($ where_clause : tt) * $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? where $ ($ where_clause) * { $ introspect ; emit_message ! ("🚀 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics_where - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen1 : ident , $ gen2 : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen1 , $ gen2 > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_two_generics - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub ($ vis : ident) fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub ($ vis) fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_vis - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_vis - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ gen : ident > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ gen > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_generic_single - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident < $ lifetime : lifetime > ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name < $ lifetime > ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_lifetime - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * pub fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * pub fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: pub_non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ (#[$ attr : meta]) * fn $ name : ident ($ ($ param : tt) *) $ (-> $ ret : ty) ? $ body : block) => { $ (#[$ attr]) * fn $ name ($ ($ param) *) $ (-> $ ret) ? { $ introspect ; emit_message ! ("🚀 MARKER: non_generic - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: non_generic - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; fn $ name : ident () $ body : block) => { fn $ name () { $ introspect ; emit_message ! ("🚀 MARKER: simple - {}" , stringify ! ($ name)) ; let result = (|| $ body) () ; emit_message ! ("🎯 MARKER: simple - {}" , stringify ! ($ name)) ; result } } ; ($ introspect : expr ; $ ($ anything : tt) *) => { $ ($ anything) * } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { () } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_compiler { ($ crate_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ crate_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_" , $ file , ".rs")) ; } ; ($ crate_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_compiler_rustc_" , $ crate_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_rust_library { ($ lib_name : literal , $ subpath : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_" , $ subpath , "_" , $ file , ".rs")) ; } ; ($ lib_name : literal , $ file : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_" , $ file , ".rs")) ; } ; ($ lib_name : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_submodules_rust_library_" , $ lib_name , "_src_lib.rs")) ; } ; }}
mkitem!{#[macro_export] macro_rules ! include_processed { ($ path : literal) => { include ! (concat ! (env ! ("CARGO_MANIFEST_DIR") , "/processed_" , $ path , ".rs")) ; } ; }}
mkitem!{macro_rules ! mkinclude { ($ path : ident) => { } ; ($ path : literal) => { include ! ($ path) } ; }}
mkitem!{macro_rules ! mkitem { (include ! ($ path : ident) ;) => { } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ string_lit : literal }) => { $ macro_name :: $ macro_sub ! { $ string_lit } } ; ($ macro_name : ident :: $ macro_sub : ident ! { $ ($ args : tt) * }) => { $ macro_name :: $ macro_sub ! { $ ($ args) * } } ; ($ macro_name : ident ! { $ ($ args : tt) * }) => { $ macro_name ! { $ ($ args) * } } ; ($ item : item) => { $ item } ; }}
mkitem!{#[macro_export] macro_rules ! mkmod { ($ name : ident , { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (pub mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; pub mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; (mod $ name : ident { $ ($ content : tt) * }) => { emit_message ! ("MOD|{}|{}" , module_path ! () , stringify ! ($ name)) ; mod $ name { const MODULE_NAME : & str = stringify ! ($ name) ; $ ($ content) * } } ; }}
mkitem!{#[macro_export] macro_rules ! mkuse { ($ use_stmt : item) => { emit_message ! ("USE|{}|{}" , module_path ! () , stringify ! ($ use_stmt)) ; $ use_stmt } ; }}
mkitem!{macro_rules ! mkstruct { ($ struct_def : item) => { $ struct_def } ; }}
mkitem!{macro_rules ! mkenum { ($ enum_def : item) => { $ enum_def } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { stringify ! ($ name) } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { "processed file" } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { "processed_path" } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { vec ! [] } ; }}
mkmod!{rustc_complete, { 
                getname!(rustc_complete);
                getsrc!(rustc_complete);
                getpath!(rustc_complete);
                get_deps!(rustc_complete);
                get_crates!(rustc_complete);
                mkinclude!(rustc_complete);
                mkmod!{emitter, { 
                getname!(emitter);
                getsrc!(emitter);
                getpath!(emitter);
                get_deps!(emitter);
                get_crates!(emitter);
                mkinclude!(emitter);
                
macro_rules! stderr_destination_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stderr_destination in module {}", module_path!());
    };
}

mkfn!{
    stderr_destination_introspect!();
    pub fn stderr_destination () { }
} 
            }}
mkmod!{registry, { 
                getname!(registry);
                getsrc!(registry);
                getpath!(registry);
                get_deps!(registry);
                get_crates!(registry);
                mkinclude!(registry);
                mkitem!{mkstruct!{pub struct Registry ;}} 
            }}
mkmod!{translation, { 
                getname!(translation);
                getsrc!(translation);
                getpath!(translation);
                get_deps!(translation);
                get_crates!(translation);
                mkinclude!(translation);
                mkitem!{mkstruct!{pub struct Translator ;}} 
            }}
mkitem!{mkstruct!{pub struct ColorConfig ;}}
mkitem!{mkstruct!{pub struct DiagCtxt ;}}
mkitem!{mkstruct!{pub struct ErrCode ;}}
mkitem!{mkstruct!{pub struct FatalError ;}}
mkitem!{mkstruct!{pub struct PResult < T > (pub T) ;}}
mkmod!{markdown, { 
                getname!(markdown);
                getsrc!(markdown);
                getpath!(markdown);
                get_deps!(markdown);
                get_crates!(markdown);
                mkinclude!(markdown);
                 
            }}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                mkitem!{mkstruct!{pub struct CG_OPTIONS ;}}
mkitem!{mkstruct!{pub struct CrateType ;}}
mkitem!{mkstruct!{pub struct ErrorOutputType ;}}
mkitem!{mkstruct!{pub struct Input ;}}
mkitem!{mkstruct!{pub struct OptionDesc ;}}
mkitem!{mkstruct!{pub struct OutFileName ;}}
mkitem!{mkstruct!{pub struct OutputType ;}}
mkitem!{mkstruct!{pub struct Sysroot ;}}
mkitem!{mkstruct!{pub struct UnstableOptions ;}}
mkitem!{mkstruct!{pub struct Z_OPTIONS ;}}

macro_rules! nightly_options_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function nightly_options in module {}", module_path!());
    };
}

mkfn!{
    nightly_options_introspect!();
    pub fn nightly_options () { }
}

macro_rules! parse_target_triple_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_target_triple in module {}", module_path!());
    };
}

mkfn!{
    parse_target_triple_introspect!();
    pub fn parse_target_triple () { }
} 
            }}
mkmod!{getopts, { 
                getname!(getopts);
                getsrc!(getopts);
                getpath!(getopts);
                get_deps!(getopts);
                get_crates!(getopts);
                mkinclude!(getopts);
                mkitem!{mkstruct!{pub struct Matches ;}} 
            }}
mkmod!{lint, { 
                getname!(lint);
                getsrc!(lint);
                getpath!(lint);
                get_deps!(lint);
                get_crates!(lint);
                mkinclude!(lint);
                mkitem!{mkstruct!{pub struct Lint ;}}
mkitem!{mkstruct!{pub struct LintId ;}} 
            }}
mkmod!{output, { 
                getname!(output);
                getsrc!(output);
                getpath!(output);
                get_deps!(output);
                get_crates!(output);
                mkinclude!(output);
                mkitem!{mkstruct!{pub struct CRATE_TYPES ;}}

macro_rules! collect_crate_types_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_types in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_types_introspect!();
    pub fn collect_crate_types () { }
}

macro_rules! invalid_output_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_output_for_target in module {}", module_path!());
    };
}

mkfn!{
    invalid_output_for_target_introspect!();
    pub fn invalid_output_for_target () { }
} 
            }}
mkitem!{mkstruct!{pub struct EarlyDiagCtxt ;}}
mkitem!{mkstruct!{pub struct Session ;}}
mkitem!{mkstruct!{pub struct FileName ;}}
mkmod!{def_id, { 
                getname!(def_id);
                getsrc!(def_id);
                getpath!(def_id);
                get_deps!(def_id);
                get_crates!(def_id);
                mkinclude!(def_id);
                mkitem!{mkstruct!{pub struct LOCAL_CRATE ;}} 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                mkitem!{mkstruct!{pub struct TyCtxt < T > (pub T) ;}} 
            }} 
            }}
mkmod!{session_diagnostics, { 
                getname!(session_diagnostics);
                getsrc!(session_diagnostics);
                getpath!(session_diagnostics);
                get_deps!(session_diagnostics);
                get_crates!(session_diagnostics);
                mkinclude!(session_diagnostics);
                mkitem!{mkstruct!{pub struct CantEmitMIR ;}}
mkitem!{mkstruct!{pub struct RLinkEmptyVersionNumber ;}}
mkitem!{mkstruct!{pub struct RLinkEncodingVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkRustcVersionMismatch ;}}
mkitem!{mkstruct!{pub struct RLinkWrongFileType ;}}
mkitem!{mkstruct!{pub struct RlinkCorruptFile ;}}
mkitem!{mkstruct!{pub struct RlinkNotAFile ;}}
mkitem!{mkstruct!{pub struct RlinkUnableToRead ;}}
mkitem!{mkstruct!{pub struct UnstableFeatureUsage ;}} 
            }}
mkitem!{macro_rules ! do_not_use_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use print") } ; }}
mkitem!{macro_rules ! do_not_use_safe_print { ($ ($ t : tt) *) => { compile_error ! ("Don't use safe_print") } ; }}
mkitem!{macro_rules ! mktrait { ($ trait_def : item) => { $ trait_def } ; }}
mkitem!{macro_rules ! mkimpl { ($ impl_def : item) => { $ impl_def } ; }}
mkitem!{macro_rules ! getname { ($ name : ident) => { pub fn get_module_name () -> &'static str { stringify ! ($ name) } } ; }}
mkitem!{macro_rules ! getsrc { ($ name : ident) => { pub fn get_source_info () -> &'static str { concat ! ("Module: " , stringify ! ($ name)) } } ; }}
mkitem!{macro_rules ! getpath { ($ name : ident) => { pub fn get_module_path () -> &'static str { module_path ! () } } ; }}
mkitem!{macro_rules ! get_deps { ($ name : ident) => { pub fn get_dependencies () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! get_crates { ($ name : ident) => { pub fn get_required_crates () -> &'static [&'static str] { & [] } } ; }}
mkitem!{macro_rules ! forall_crates { ($ ($ crate_name : ident) ,*) => { $ (extern crate $ crate_name ;) * } ; }}
mkitem!{macro_rules ! emit_extern { ($ crate_name : ident) => { extern crate $ crate_name ; } ; }}
mkitem!{macro_rules ! get_externs { ($ crate_name : ident) => { stringify ! ($ crate_name) } ; }}
mkuse!{use core :: { cmp , fmt , ops } ;}
mkitem!{mktrait!{#[doc = " Minimal integer implementations needed on all integer types, including wide integers."] #[allow (dead_code)] pub trait MinInt : Copy + fmt :: Debug + ops :: BitOr < Output = Self > + ops :: Not < Output = Self > + ops :: Shl < u32 , Output = Self > { #[doc = " Type with the same width but other signedness"] type OtherSign : MinInt ; #[doc = " Unsigned version of Self"] type Unsigned : MinInt ; #[doc = " If `Self` is a signed integer"] const SIGNED : bool ; #[doc = " The bitwidth of the int type"] const BITS : u32 ; const ZERO : Self ; const ONE : Self ; const MIN : Self ; const MAX : Self ; }}}
mkitem!{#[doc = " Access the associated `OtherSign` type from an int (helper to avoid ambiguous associated"] #[doc = " types)."] pub type OtherSign < I > = < I as MinInt > :: OtherSign ;}
mkitem!{mktrait!{#[doc = " Trait for some basic operations on integers"] #[allow (dead_code)] pub trait Int : MinInt + fmt :: Display + fmt :: Binary + fmt :: LowerHex + ops :: AddAssign + ops :: SubAssign + ops :: MulAssign + ops :: DivAssign + ops :: RemAssign + ops :: BitAndAssign + ops :: BitOrAssign + ops :: BitXorAssign + ops :: ShlAssign < i32 > + ops :: ShlAssign < u32 > + ops :: ShrAssign < u32 > + ops :: ShrAssign < i32 > + ops :: Add < Output = Self > + ops :: Sub < Output = Self > + ops :: Mul < Output = Self > + ops :: Div < Output = Self > + ops :: Rem < Output = Self > + ops :: Shl < i32 , Output = Self > + ops :: Shl < u32 , Output = Self > + ops :: Shr < i32 , Output = Self > + ops :: Shr < u32 , Output = Self > + ops :: BitXor < Output = Self > + ops :: BitAnd < Output = Self > + cmp :: Ord + From < bool > + CastFrom < i32 > + CastFrom < u16 > + CastFrom < u32 > + CastFrom < u8 > + CastFrom < usize > + CastInto < i32 > + CastInto < u16 > + CastInto < u32 > + CastInto < u8 > + CastInto < usize > { fn signed (self) -> OtherSign < Self :: Unsigned > ; fn unsigned (self) -> Self :: Unsigned ; fn from_unsigned (unsigned : Self :: Unsigned) -> Self ; fn abs (self) -> Self ; fn unsigned_abs (self) -> Self :: Unsigned ; fn from_bool (b : bool) -> Self ; #[doc = " Prevents the need for excessive conversions between signed and unsigned"] fn logical_shr (self , other : u32) -> Self ; #[doc = " Absolute difference between two integers."] fn abs_diff (self , other : Self) -> Self :: Unsigned ; fn is_zero (self) -> bool ; fn checked_add (self , other : Self) -> Option < Self > ; fn checked_sub (self , other : Self) -> Option < Self > ; fn wrapping_neg (self) -> Self ; fn wrapping_add (self , other : Self) -> Self ; fn wrapping_mul (self , other : Self) -> Self ; fn wrapping_sub (self , other : Self) -> Self ; fn wrapping_shl (self , other : u32) -> Self ; fn wrapping_shr (self , other : u32) -> Self ; fn rotate_left (self , other : u32) -> Self ; fn overflowing_add (self , other : Self) -> (Self , bool) ; fn overflowing_sub (self , other : Self) -> (Self , bool) ; fn carrying_add (self , other : Self , carry : bool) -> (Self , bool) ; fn borrowing_sub (self , other : Self , borrow : bool) -> (Self , bool) ; fn leading_zeros (self) -> u32 ; fn trailing_zeros (self) -> u32 ; fn ilog2 (self) -> u32 ; }}}
mkitem!{macro_rules ! int_impl_common { ($ ty : ty) => { fn from_bool (b : bool) -> Self { b as $ ty } fn logical_shr (self , other : u32) -> Self { Self :: from_unsigned (self . unsigned () . wrapping_shr (other)) } fn is_zero (self) -> bool { self == Self :: ZERO } fn checked_add (self , other : Self) -> Option < Self > { self . checked_add (other) } fn checked_sub (self , other : Self) -> Option < Self > { self . checked_sub (other) } fn wrapping_neg (self) -> Self { < Self >:: wrapping_neg (self) } fn wrapping_add (self , other : Self) -> Self { < Self >:: wrapping_add (self , other) } fn wrapping_mul (self , other : Self) -> Self { < Self >:: wrapping_mul (self , other) } fn wrapping_sub (self , other : Self) -> Self { < Self >:: wrapping_sub (self , other) } fn wrapping_shl (self , other : u32) -> Self { < Self >:: wrapping_shl (self , other) } fn wrapping_shr (self , other : u32) -> Self { < Self >:: wrapping_shr (self , other) } fn rotate_left (self , other : u32) -> Self { < Self >:: rotate_left (self , other) } fn overflowing_add (self , other : Self) -> (Self , bool) { < Self >:: overflowing_add (self , other) } fn overflowing_sub (self , other : Self) -> (Self , bool) { < Self >:: overflowing_sub (self , other) } fn leading_zeros (self) -> u32 { < Self >:: leading_zeros (self) } fn trailing_zeros (self) -> u32 { < Self >:: trailing_zeros (self) } fn ilog2 (self) -> u32 { #[allow (clippy :: incompatible_msrv)] < Self >:: ilog2 (self) } fn carrying_add (self , other : Self , carry : bool) -> (Self , bool) { let (ab , of1) = self . overflowing_add (other) ; let (abc , of2) = ab . overflowing_add (Self :: from_bool (carry)) ; (abc , of1 ^ of2) } fn borrowing_sub (self , other : Self , borrow : bool) -> (Self , bool) { let (ab , of1) = self . overflowing_sub (other) ; let (abc , of2) = ab . overflowing_sub (Self :: from_bool (borrow)) ; (abc , of1 ^ of2) } } ; }}
mkitem!{macro_rules ! int_impl { ($ ity : ty , $ uty : ty) => { impl MinInt for $ uty { type OtherSign = $ ity ; type Unsigned = $ uty ; const BITS : u32 = < Self as MinInt >:: ZERO . count_zeros () ; const SIGNED : bool = Self :: MIN != Self :: ZERO ; const ZERO : Self = 0 ; const ONE : Self = 1 ; const MIN : Self = < Self >:: MIN ; const MAX : Self = < Self >:: MAX ; } impl Int for $ uty { fn signed (self) -> $ ity { self as $ ity } fn unsigned (self) -> Self { self } fn abs (self) -> Self { unimplemented ! () } fn unsigned_abs (self) -> Self { unimplemented ! () } #[allow (clippy :: wrong_self_convention)] fn from_unsigned (me : $ uty) -> Self { me } fn abs_diff (self , other : Self) -> Self { self . abs_diff (other) } int_impl_common ! ($ uty) ; } impl MinInt for $ ity { type OtherSign = $ uty ; type Unsigned = $ uty ; const BITS : u32 = < Self as MinInt >:: ZERO . count_zeros () ; const SIGNED : bool = Self :: MIN != Self :: ZERO ; const ZERO : Self = 0 ; const ONE : Self = 1 ; const MIN : Self = < Self >:: MIN ; const MAX : Self = < Self >:: MAX ; } impl Int for $ ity { fn signed (self) -> Self { self } fn unsigned (self) -> $ uty { self as $ uty } fn abs (self) -> Self { self . abs () } fn unsigned_abs (self) -> Self :: Unsigned { self . unsigned_abs () } fn from_unsigned (me : $ uty) -> Self { me as $ ity } fn abs_diff (self , other : Self) -> $ uty { self . abs_diff (other) } int_impl_common ! ($ ity) ; } } ; }}
mkitem!{int_impl ! (isize , usize) ;}
mkitem!{int_impl ! (i8 , u8) ;}
mkitem!{int_impl ! (i16 , u16) ;}
mkitem!{int_impl ! (i32 , u32) ;}
mkitem!{int_impl ! (i64 , u64) ;}
mkitem!{int_impl ! (i128 , u128) ;}
mkitem!{mktrait!{#[doc = " Trait for integers twice the bit width of another integer. This is implemented for all"] #[doc = " primitives except for `u8`, because there is not a smaller primitive."] pub trait DInt : MinInt { #[doc = " Integer that is half the bit width of the integer this trait is implemented for"] type H : HInt < D = Self > ; #[doc = " Returns the low half of `self`"] fn lo (self) -> Self :: H ; #[doc = " Returns the high half of `self`"] fn hi (self) -> Self :: H ; #[doc = " Returns the low and high halves of `self` as a tuple"] fn lo_hi (self) -> (Self :: H , Self :: H) { (self . lo () , self . hi ()) } #[doc = " Constructs an integer using lower and higher half parts"] #[allow (unused)] fn from_lo_hi (lo : Self :: H , hi : Self :: H) -> Self { lo . zero_widen () | hi . widen_hi () } }}}
mkitem!{mktrait!{#[doc = " Trait for integers half the bit width of another integer. This is implemented for all"] #[doc = " primitives except for `u128`, because it there is not a larger primitive."] pub trait HInt : Int { #[doc = " Integer that is double the bit width of the integer this trait is implemented for"] type D : DInt < H = Self > + MinInt ; #[doc = " Widens (using default extension) the integer to have double bit width"] fn widen (self) -> Self :: D ; #[doc = " Widens (zero extension only) the integer to have double bit width. This is needed to get"] #[doc = " around problems with associated type bounds (such as `Int<Othersign: DInt>`) being unstable"] fn zero_widen (self) -> Self :: D ; #[doc = " Widens the integer to have double bit width and shifts the integer into the higher bits"] #[allow (unused)] fn widen_hi (self) -> Self :: D ; #[doc = " Widening multiplication with zero widening. This cannot overflow."] fn zero_widen_mul (self , rhs : Self) -> Self :: D ; #[doc = " Widening multiplication. This cannot overflow."] fn widen_mul (self , rhs : Self) -> Self :: D ; }}}
mkitem!{macro_rules ! impl_d_int { ($ ($ X : ident $ D : ident) ,*) => { $ (impl DInt for $ D { type H = $ X ; fn lo (self) -> Self :: H { self as $ X } fn hi (self) -> Self :: H { (self >> <$ X as MinInt >:: BITS) as $ X } }) * } ; }}
mkitem!{macro_rules ! impl_h_int { ($ ($ H : ident $ uH : ident $ X : ident) ,*) => { $ (impl HInt for $ H { type D = $ X ; fn widen (self) -> Self :: D { self as $ X } fn zero_widen (self) -> Self :: D { (self as $ uH) as $ X } fn zero_widen_mul (self , rhs : Self) -> Self :: D { self . zero_widen () . wrapping_mul (rhs . zero_widen ()) } fn widen_mul (self , rhs : Self) -> Self :: D { self . widen () . wrapping_mul (rhs . widen ()) } fn widen_hi (self) -> Self :: D { (self as $ X) << < Self as MinInt >:: BITS } }) * } ; }}
mkitem!{impl_d_int ! (u8 u16 , u16 u32 , u32 u64 , u64 u128 , i8 i16 , i16 i32 , i32 i64 , i64 i128) ;}
mkitem!{impl_h_int ! (u8 u8 u16 , u16 u16 u32 , u32 u32 u64 , u64 u64 u128 , i8 u8 i16 , i16 u16 i32 , i32 u32 i64 , i64 u64 i128) ;}
mkitem!{mktrait!{#[doc = " Trait to express (possibly lossy) casting of integers"] pub trait CastInto < T : Copy > : Copy { #[doc = " By default, casts should be exact."] #[track_caller] fn cast (self) -> T ; #[doc = " Call for casts that are expected to truncate."] #[doc = ""] #[doc = " In practice, this is exactly the same as `cast`; the main difference is to document intent"] #[doc = " in code. `cast` may panic in debug mode."] fn cast_lossy (self) -> T ; }}}
mkitem!{mktrait!{pub trait CastFrom < T : Copy > : Copy { #[doc = " By default, casts should be exact."] #[track_caller] fn cast_from (value : T) -> Self ; #[doc = " Call for casts that are expected to truncate."] fn cast_from_lossy (value : T) -> Self ; }}}
mkitem!{mkimpl!{impl < T : Copy , U : CastInto < T > + Copy > CastFrom < U > for T { fn cast_from (value : U) -> Self { value . cast () } fn cast_from_lossy (value : U) -> Self { value . cast_lossy () } }}}
mkitem!{macro_rules ! cast_into { ($ ty : ty) => { cast_into ! ($ ty ; usize , isize , u8 , i8 , u16 , i16 , u32 , i32 , u64 , i64 , u128 , i128) ; } ; ($ ty : ty ; $ ($ into : ty) ,*) => { $ (impl CastInto <$ into > for $ ty { fn cast (self) -> $ into { #[cfg (not (feature = "compiler-builtins"))] debug_assert ! (<$ into >:: try_from (self) . is_ok () , "failed cast from {self}") ; self as $ into } fn cast_lossy (self) -> $ into { self as $ into } }) * } ; }}
mkitem!{macro_rules ! cast_into_float { ($ ty : ty) => { #[cfg (f16_enabled)] cast_into_float ! ($ ty ; f16) ; cast_into_float ! ($ ty ; f32 , f64) ; #[cfg (f128_enabled)] cast_into_float ! ($ ty ; f128) ; } ; ($ ty : ty ; $ ($ into : ty) ,*) => { $ (impl CastInto <$ into > for $ ty { fn cast (self) -> $ into { #[cfg (not (feature = "compiler-builtins"))] debug_assert_eq ! (self as $ into as $ ty , self , "inexact float cast") ; self as $ into } fn cast_lossy (self) -> $ into { self as $ into } }) * } ; }}
mkitem!{cast_into ! (usize) ;}
mkitem!{cast_into ! (isize) ;}
mkitem!{cast_into ! (u8) ;}
mkitem!{cast_into ! (i8) ;}
mkitem!{cast_into ! (u16) ;}
mkitem!{cast_into ! (i16) ;}
mkitem!{cast_into ! (u32) ;}
mkitem!{cast_into ! (i32) ;}
mkitem!{cast_into ! (u64) ;}
mkitem!{cast_into ! (i64) ;}
mkitem!{cast_into ! (u128) ;}
mkitem!{cast_into ! (i128) ;}
mkitem!{cast_into_float ! (i8) ;}
mkitem!{cast_into_float ! (i16) ;}
mkitem!{cast_into_float ! (i32) ;}
mkitem!{cast_into_float ! (i64) ;}
mkitem!{cast_into_float ! (i128) ;}