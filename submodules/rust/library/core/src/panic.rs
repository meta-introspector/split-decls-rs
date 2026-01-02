mkmod!{location, { 
                getname!(location);
                getsrc!(location);
                getpath!(location);
                get_deps!(location);
                get_crates!(location);
                mkinclude!(location);
                 
            }}
mkmod!{panic_info, { 
                getname!(panic_info);
                getsrc!(panic_info);
                getpath!(panic_info);
                get_deps!(panic_info);
                get_crates!(panic_info);
                mkinclude!(panic_info);
                 
            }}
mkmod!{unwind_safe, { 
                getname!(unwind_safe);
                getsrc!(unwind_safe);
                getpath!(unwind_safe);
                get_deps!(unwind_safe);
                get_crates!(unwind_safe);
                mkinclude!(unwind_safe);
                 
            }}
mkuse!{# [stable (feature = "panic_hooks" , since = "1.10.0")] pub use self :: location :: Location ;}
mkuse!{# [stable (feature = "panic_hooks" , since = "1.10.0")] pub use self :: panic_info :: PanicInfo ;}
mkuse!{# [stable (feature = "panic_info_message" , since = "1.81.0")] pub use self :: panic_info :: PanicMessage ;}
mkuse!{# [stable (feature = "catch_unwind" , since = "1.9.0")] pub use self :: unwind_safe :: { AssertUnwindSafe , RefUnwindSafe , UnwindSafe } ;}
mkuse!{use crate :: any :: Any ;}
mkitem!{# [doc (hidden)] # [unstable (feature = "edition_panic" , issue = "none" , reason = "use panic!() instead")] # [allow_internal_unstable (panic_internals , const_format_args)] # [rustc_diagnostic_item = "core_panic_2015_macro"] # [rustc_macro_transparency = "semitransparent"] pub macro panic_2015 { () => ($ crate :: panicking :: panic ("explicit panic")) , ($ msg : literal $ (,) ?) => ($ crate :: panicking :: panic ($ msg)) , ($ msg : expr $ (,) ?) => ({ $ crate :: panicking :: panic_str_2015 ($ msg) ; }) , ("{}" , $ arg : expr $ (,) ?) => ({ $ crate :: panicking :: panic_display (&$ arg) ; }) , ($ fmt : expr , $ ($ arg : tt) +) => ({ $ crate :: panicking :: panic_fmt ($ crate :: const_format_args ! ($ fmt , $ ($ arg) +)) ; }) , }}
mkitem!{# [doc (hidden)] # [unstable (feature = "edition_panic" , issue = "none" , reason = "use panic!() instead")] # [allow_internal_unstable (panic_internals , const_format_args)] # [rustc_diagnostic_item = "core_panic_2021_macro"] # [rustc_macro_transparency = "semitransparent"] pub macro panic_2021 { () => ($ crate :: panicking :: panic ("explicit panic")) , ("{}" , $ arg : expr $ (,) ?) => ({ $ crate :: panicking :: panic_display (&$ arg) ; }) , ($ ($ t : tt) +) => ({ $ crate :: panicking :: panic_fmt ($ crate :: const_format_args ! ($ ($ t) +)) ; }) , }}
mkitem!{# [doc (hidden)] # [unstable (feature = "edition_panic" , issue = "none" , reason = "use unreachable!() instead")] # [allow_internal_unstable (panic_internals)] # [rustc_diagnostic_item = "unreachable_2015_macro"] # [rustc_macro_transparency = "semitransparent"] pub macro unreachable_2015 { () => ($ crate :: panicking :: panic ("internal error: entered unreachable code")) , ($ msg : expr $ (,) ?) => ({ $ crate :: panicking :: unreachable_display (&$ msg) ; }) , ($ fmt : expr , $ ($ arg : tt) *) => ($ crate :: panic ! ($ crate :: concat ! ("internal error: entered unreachable code: " , $ fmt) , $ ($ arg) *)) , }}
mkitem!{# [doc (hidden)] # [unstable (feature = "edition_panic" , issue = "none" , reason = "use unreachable!() instead")] # [allow_internal_unstable (panic_internals)] # [rustc_macro_transparency = "semitransparent"] pub macro unreachable_2021 { () => ($ crate :: panicking :: panic ("internal error: entered unreachable code")) , ($ ($ t : tt) +) => ($ crate :: panic ! ("internal error: entered unreachable code: {}" , $ crate :: format_args ! ($ ($ t) +))) , }}

macro_rules! abort_unwind_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_unwind in module {}", module_path!());
    };
}

mkfn!{
    abort_unwind_introspect!();
    # [doc = " Invokes a closure, aborting if the closure unwinds."] # [doc = ""] # [doc = " When compiled with aborting panics, this function is effectively a no-op."] # [doc = " With unwinding panics, an unwind results in another call into the panic"] # [doc = " hook followed by a process abort."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Instead of using this function, code should attempt to support unwinding."] # [doc = " Implementing [`Drop`] allows you to restore invariants uniformly in both"] # [doc = " return and unwind paths."] # [doc = ""] # [doc = " If an unwind can lead to logical issues but not soundness issues, you"] # [doc = " should allow the unwind. Opting out of [`UnwindSafe`] indicates to your"] # [doc = " consumers that they need to consider correctness in the face of unwinds."] # [doc = ""] # [doc = " If an unwind would be unsound, then this function should be used in order"] # [doc = " to prevent unwinds. However, note that `extern \"C\" fn` will automatically"] # [doc = " convert unwinds to aborts, so using this function isn't necessary for FFI."] # [unstable (feature = "abort_unwind" , issue = "130338")] # [rustc_nounwind] pub fn abort_unwind < F : FnOnce () -> R , R > (f : F) -> R { f () }
}
mkitem!{mktrait!{# [doc = " An internal trait used by std to pass data from std to `panic_unwind` and"] # [doc = " other panic runtimes. Not intended to be stabilized any time soon, do not"] # [doc = " use."] # [unstable (feature = "std_internals" , issue = "none")] # [doc (hidden)] pub unsafe trait PanicPayload : crate :: fmt :: Display { # [doc = " Take full ownership of the contents."] # [doc = " The return type is actually `Box<dyn Any + Send>`, but we cannot use `Box` in core."] # [doc = ""] # [doc = " After this method got called, only some dummy default value is left in `self`."] # [doc = " Calling this method twice, or calling `get` after calling this method, is an error."] # [doc = ""] # [doc = " The argument is borrowed because the panic runtime (`__rust_start_panic`) only"] # [doc = " gets a borrowed `dyn PanicPayload`."] fn take_box (& mut self) -> * mut (dyn Any + Send) ; # [doc = " Just borrow the contents."] fn get (& mut self) -> & (dyn Any + Send) ; # [doc = " Tries to borrow the contents as `&str`, if possible without doing any allocations."] fn as_str (& mut self) -> Option < & str > { None } }}}
mkitem!{# [doc = " Helper macro for panicking in a `const fn`."] # [doc = " Invoke as:"] # [doc = " ```rust,ignore (just an example)"] # [doc = " core::macros::const_panic!(\"boring message\", \"flavored message {a} {b:?}\", a: u32 = foo.len(), b: Something = bar);"] # [doc = " ```"] # [doc = " where the first message will be printed in const-eval,"] # [doc = " and the second message will be printed at runtime."] # [unstable (feature = "panic_internals" , issue = "none")] # [doc (hidden)] pub macro const_panic { ($ const_msg : literal , $ runtime_msg : literal , $ ($ arg : ident : $ ty : ty = $ val : expr) ,* $ (,) ?) => { { # [rustc_allow_const_fn_unstable (const_eval_select)] # [inline (always)] # [track_caller] const fn do_panic ($ ($ arg : $ ty) ,*) -> ! { $ crate :: intrinsics :: const_eval_select ! (@ capture { $ ($ arg : $ ty = $ arg) ,* } -> !: # [noinline] if const # [track_caller] # [inline] { $ crate :: panic ! ($ const_msg) } else # [track_caller] { $ crate :: panic ! ($ runtime_msg) }) } do_panic ($ ($ val) ,*) } } , ($ const_msg : literal , $ runtime_msg : literal , $ ($ arg : ident : $ ty : ty) ,* $ (,) ?) => { $ crate :: panic :: const_panic ! ($ const_msg , $ runtime_msg , $ ($ arg : $ ty = $ arg) ,*) } , }}
mkitem!{# [doc = " A version of `assert` that prints a non-formatting message in const contexts."] # [doc = ""] # [doc = " See [`const_panic!`]."] # [unstable (feature = "panic_internals" , issue = "none")] # [doc (hidden)] pub macro const_assert { ($ condition : expr , $ const_msg : literal , $ runtime_msg : literal , $ ($ arg : tt) *) => { { if !$ crate :: intrinsics :: likely ($ condition) { $ crate :: panic :: const_panic ! ($ const_msg , $ runtime_msg , $ ($ arg) *) } } } }}