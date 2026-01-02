mkuse!{use crate :: fmt ;}
mkuse!{use crate :: intrinsics :: const_eval_select ;}
mkuse!{use crate :: panic :: { Location , PanicInfo } ;}
mkitem!{# [cfg (feature = "panic_immediate_abort")] const _ : () = assert ! (cfg ! (panic = "abort") , "panic_immediate_abort requires -C panic=abort") ;}

macro_rules! panic_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_fmt in module {}", module_path!());
    };
}

mkfn!{
    panic_fmt_introspect!();
    # [doc = " The entry point for panicking with a formatted message."] # [doc = ""] # [doc = " This is designed to reduce the amount of code required at the call"] # [doc = " site as much as possible (so that `panic!()` has as low an impact"] # [doc = " on (e.g.) the inlining of other functions as possible), by moving"] # [doc = " the actual formatting into this shared place."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold)] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [lang = "panic_fmt"] # [rustc_do_not_const_check] # [rustc_const_stable_indirect] pub const fn panic_fmt (fmt : fmt :: Arguments < '_ >) -> ! { if cfg ! (feature = "panic_immediate_abort") { super :: intrinsics :: abort () } unsafe extern "Rust" { # [lang = "panic_impl"] fn panic_impl (pi : & PanicInfo < '_ >) -> ! ; } let pi = PanicInfo :: new (& fmt , Location :: caller () , true , false ,) ; unsafe { panic_impl (& pi) } }
}

macro_rules! panic_nounwind_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_nounwind_fmt in module {}", module_path!());
    };
}

mkfn!{
    panic_nounwind_fmt_introspect!();
    # [doc = " Like `panic_fmt`, but for non-unwinding panics."] # [doc = ""] # [doc = " Has to be a separate function so that it can carry the `rustc_nounwind` attribute."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold)] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [rustc_nounwind] # [rustc_const_stable_indirect] # [rustc_allow_const_fn_unstable (const_eval_select)] pub const fn panic_nounwind_fmt (fmt : fmt :: Arguments < '_ > , force_no_backtrace : bool) -> ! { const_eval_select ! (@ capture { fmt : fmt :: Arguments <'_ >, force_no_backtrace : bool } -> !: if const # [track_caller] { panic_fmt (fmt) } else # [track_caller] { if cfg ! (feature = "panic_immediate_abort") { super :: intrinsics :: abort () } unsafe extern "Rust" { # [lang = "panic_impl"] fn panic_impl (pi : & PanicInfo <'_ >) -> !; } let pi = PanicInfo :: new (& fmt , Location :: caller () , false , force_no_backtrace ,) ; unsafe { panic_impl (& pi) } }) }
}

macro_rules! panic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic in module {}", module_path!());
    };
}

mkfn!{
    panic_introspect!();
    # [doc = " The underlying implementation of core's `panic!` macro when no formatting is used."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold)] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [rustc_const_stable_indirect] # [lang = "panic"] pub const fn panic (expr : & 'static str) -> ! { panic_fmt (fmt :: Arguments :: new_const (& [expr])) ; }
}
mkitem!{macro_rules ! panic_const { ($ ($ lang : ident = $ message : expr ,) +) => { $ (# [doc = " This is a panic called with a message that's a result of a MIR-produced Assert."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold)] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [rustc_const_stable_indirect] # [lang = stringify ! ($ lang)] pub const fn $ lang () -> ! { panic_fmt (fmt :: Arguments :: new_const (& [$ message])) ; }) + } }}
mkmod!{panic_const, { 
                getname!(panic_const);
                getsrc!(panic_const);
                getpath!(panic_const);
                get_deps!(panic_const);
                get_crates!(panic_const);
                mkinclude!(panic_const);
                mkuse!{use super :: * ;}
mkitem!{panic_const ! { panic_const_add_overflow = "attempt to add with overflow" , panic_const_sub_overflow = "attempt to subtract with overflow" , panic_const_mul_overflow = "attempt to multiply with overflow" , panic_const_div_overflow = "attempt to divide with overflow" , panic_const_rem_overflow = "attempt to calculate the remainder with overflow" , panic_const_neg_overflow = "attempt to negate with overflow" , panic_const_shr_overflow = "attempt to shift right with overflow" , panic_const_shl_overflow = "attempt to shift left with overflow" , panic_const_div_by_zero = "attempt to divide by zero" , panic_const_rem_by_zero = "attempt to calculate the remainder with a divisor of zero" , panic_const_coroutine_resumed = "coroutine resumed after completion" , panic_const_async_fn_resumed = "`async fn` resumed after completion" , panic_const_async_gen_fn_resumed = "`async gen fn` resumed after completion" , panic_const_gen_fn_none = "`gen fn` should just keep returning `None` after completion" , panic_const_coroutine_resumed_panic = "coroutine resumed after panicking" , panic_const_async_fn_resumed_panic = "`async fn` resumed after panicking" , panic_const_async_gen_fn_resumed_panic = "`async gen fn` resumed after panicking" , panic_const_gen_fn_none_panic = "`gen fn` should just keep returning `None` after panicking" , }}
mkitem!{panic_const ! { panic_const_coroutine_resumed_drop = "coroutine resumed after async drop" , panic_const_async_fn_resumed_drop = "`async fn` resumed after async drop" , panic_const_async_gen_fn_resumed_drop = "`async gen fn` resumed after async drop" , panic_const_gen_fn_none_drop = "`gen fn` resumed after async drop" , }} 
            }}

macro_rules! panic_nounwind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_nounwind in module {}", module_path!());
    };
}

mkfn!{
    panic_nounwind_introspect!();
    # [doc = " Like `panic`, but without unwinding and track_caller to reduce the impact on codesize on the caller."] # [doc = " If you want `#[track_caller]` for nicer errors, call `panic_nounwind_fmt` directly."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold)] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [lang = "panic_nounwind"] # [rustc_nounwind] # [rustc_const_stable_indirect] pub const fn panic_nounwind (expr : & 'static str) -> ! { panic_nounwind_fmt (fmt :: Arguments :: new_const (& [expr]) , false) ; }
}

macro_rules! panic_nounwind_nobacktrace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_nounwind_nobacktrace in module {}", module_path!());
    };
}

mkfn!{
    panic_nounwind_nobacktrace_introspect!();
    # [doc = " Like `panic_nounwind`, but also inhibits showing a backtrace."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold)] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [rustc_nounwind] pub fn panic_nounwind_nobacktrace (expr : & 'static str) -> ! { panic_nounwind_fmt (fmt :: Arguments :: new_const (& [expr]) , true) ; }
}

macro_rules! unreachable_display_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unreachable_display in module {}", module_path!());
    };
}

mkfn!{
    unreachable_display_introspect!();
    # [inline] # [track_caller] # [rustc_diagnostic_item = "unreachable_display"] pub fn unreachable_display < T : fmt :: Display > (x : & T) -> ! { panic_fmt (format_args ! ("internal error: entered unreachable code: {}" , * x)) ; }
}

macro_rules! panic_str_2015_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_str_2015 in module {}", module_path!());
    };
}

mkfn!{
    panic_str_2015_introspect!();
    # [doc = " This exists solely for the 2015 edition `panic!` macro to trigger"] # [doc = " a lint on `panic!(my_str_variable);`."] # [inline] # [track_caller] # [rustc_diagnostic_item = "panic_str_2015"] # [rustc_const_stable_indirect] pub const fn panic_str_2015 (expr : & str) -> ! { panic_display (& expr) ; }
}

macro_rules! panic_display_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_display in module {}", module_path!());
    };
}

mkfn!{
    panic_display_introspect!();
    # [inline] # [track_caller] # [lang = "panic_display"] # [rustc_do_not_const_check] # [rustc_const_stable_indirect] pub const fn panic_display < T : fmt :: Display > (x : & T) -> ! { panic_fmt (format_args ! ("{}" , * x)) ; }
}

macro_rules! panic_bounds_check_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_bounds_check in module {}", module_path!());
    };
}

mkfn!{
    panic_bounds_check_introspect!();
    # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [lang = "panic_bounds_check"] fn panic_bounds_check (index : usize , len : usize) -> ! { if cfg ! (feature = "panic_immediate_abort") { super :: intrinsics :: abort () } panic ! ("index out of bounds: the len is {len} but the index is {index}") }
}

macro_rules! panic_misaligned_pointer_dereference_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_misaligned_pointer_dereference in module {}", module_path!());
    };
}

mkfn!{
    panic_misaligned_pointer_dereference_introspect!();
    # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [lang = "panic_misaligned_pointer_dereference"] # [rustc_nounwind] fn panic_misaligned_pointer_dereference (required : usize , found : usize) -> ! { if cfg ! (feature = "panic_immediate_abort") { super :: intrinsics :: abort () } panic_nounwind_fmt (format_args ! ("misaligned pointer dereference: address must be a multiple of {required:#x} but is {found:#x}") , false ,) }
}

macro_rules! panic_null_pointer_dereference_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_null_pointer_dereference in module {}", module_path!());
    };
}

mkfn!{
    panic_null_pointer_dereference_introspect!();
    # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [lang = "panic_null_pointer_dereference"] # [rustc_nounwind] fn panic_null_pointer_dereference () -> ! { if cfg ! (feature = "panic_immediate_abort") { super :: intrinsics :: abort () } panic_nounwind_fmt (format_args ! ("null pointer dereference occurred") , false ,) }
}

macro_rules! panic_invalid_enum_construction_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_invalid_enum_construction in module {}", module_path!());
    };
}

mkfn!{
    panic_invalid_enum_construction_introspect!();
    # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [lang = "panic_invalid_enum_construction"] # [rustc_nounwind] fn panic_invalid_enum_construction (source : u128) -> ! { if cfg ! (feature = "panic_immediate_abort") { super :: intrinsics :: abort () } panic_nounwind_fmt (format_args ! ("trying to construct an enum from an invalid value {source:#x}") , false ,) }
}

macro_rules! panic_cannot_unwind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_cannot_unwind in module {}", module_path!());
    };
}

mkfn!{
    panic_cannot_unwind_introspect!();
    # [doc = " Panics because we cannot unwind out of a function."] # [doc = ""] # [doc = " This is a separate function to avoid the codesize impact of each crate containing the string to"] # [doc = " pass to `panic_nounwind`."] # [doc = ""] # [doc = " This function is called directly by the codegen backend, and must not have"] # [doc = " any extra arguments (including those synthesized by track_caller)."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [lang = "panic_cannot_unwind"] # [rustc_nounwind] fn panic_cannot_unwind () -> ! { panic_nounwind ("panic in a function that cannot unwind") }
}

macro_rules! panic_in_cleanup_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_in_cleanup in module {}", module_path!());
    };
}

mkfn!{
    panic_in_cleanup_introspect!();
    # [doc = " Panics because we are unwinding out of a destructor during cleanup."] # [doc = ""] # [doc = " This is a separate function to avoid the codesize impact of each crate containing the string to"] # [doc = " pass to `panic_nounwind`."] # [doc = ""] # [doc = " This function is called directly by the codegen backend, and must not have"] # [doc = " any extra arguments (including those synthesized by track_caller)."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [lang = "panic_in_cleanup"] # [rustc_nounwind] fn panic_in_cleanup () -> ! { panic_nounwind_nobacktrace ("panic in a destructor during cleanup") }
}

macro_rules! const_panic_fmt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_panic_fmt in module {}", module_path!());
    };
}

mkfn!{
    const_panic_fmt_introspect!();
    # [doc = " This function is used instead of panic_fmt in const eval."] # [lang = "const_panic_fmt"] # [rustc_const_stable_indirect] pub const fn const_panic_fmt (fmt : fmt :: Arguments < '_ >) -> ! { if let Some (msg) = fmt . as_str () { panic_display (& msg) ; } else { unsafe { crate :: hint :: unreachable_unchecked () } ; } }
}
mkitem!{mkenum!{# [derive (Debug)] # [doc (hidden)] pub enum AssertKind { Eq , Ne , Match , }}}

macro_rules! assert_failed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_failed in module {}", module_path!());
    };
}

mkfn!{
    assert_failed_introspect!();
    # [doc = " Internal function for `assert_eq!` and `assert_ne!` macros"] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [doc (hidden)] pub fn assert_failed < T , U > (kind : AssertKind , left : & T , right : & U , args : Option < fmt :: Arguments < '_ > > ,) -> ! where T : fmt :: Debug + ? Sized , U : fmt :: Debug + ? Sized , { assert_failed_inner (kind , & left , & right , args) }
}

macro_rules! assert_matches_failed_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_matches_failed in module {}", module_path!());
    };
}

mkfn!{
    assert_matches_failed_introspect!();
    # [doc = " Internal function for `assert_match!`"] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] # [doc (hidden)] pub fn assert_matches_failed < T : fmt :: Debug + ? Sized > (left : & T , right : & str , args : Option < fmt :: Arguments < '_ > > ,) -> ! { struct Pattern < 'a > (& 'a str) ; impl fmt :: Debug for Pattern < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (self . 0) } } assert_failed_inner (AssertKind :: Match , & left , & Pattern (right) , args) ; }
}

macro_rules! assert_failed_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_failed_inner in module {}", module_path!());
    };
}

mkfn!{
    assert_failed_inner_introspect!();
    # [doc = " Non-generic version of the above functions, to avoid code bloat."] # [cfg_attr (not (feature = "panic_immediate_abort") , inline (never) , cold , optimize (size))] # [cfg_attr (feature = "panic_immediate_abort" , inline)] # [track_caller] fn assert_failed_inner (kind : AssertKind , left : & dyn fmt :: Debug , right : & dyn fmt :: Debug , args : Option < fmt :: Arguments < '_ > > ,) -> ! { let op = match kind { AssertKind :: Eq => "==" , AssertKind :: Ne => "!=" , AssertKind :: Match => "matches" , } ; match args { Some (args) => panic ! (r#"assertion `left {op} right` failed: {args}
  left: {left:?}
 right: {right:?}"#) , None => panic ! (r#"assertion `left {op} right` failed
  left: {left:?}
 right: {right:?}"#) , } }
}