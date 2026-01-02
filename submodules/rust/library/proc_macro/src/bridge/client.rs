mkuse!{use std :: cell :: RefCell ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: sync :: atomic :: AtomicU32 ;}
mkuse!{use super :: * ;}
mkitem!{macro_rules ! define_client_handles { ('owned : $ ($ oty : ident ,) * 'interned : $ ($ ity : ident ,) *) => { # [repr (C)] # [allow (non_snake_case)] pub (super) struct HandleCounters { $ (pub (super) $ oty : AtomicU32 ,) * $ (pub (super) $ ity : AtomicU32 ,) * } static COUNTERS : HandleCounters = HandleCounters { $ ($ oty : AtomicU32 :: new (1) ,) * $ ($ ity : AtomicU32 :: new (1) ,) * } ; $ (pub (crate) struct $ oty { handle : handle :: Handle , _marker : PhantomData <* mut () >, } impl Drop for $ oty { fn drop (& mut self) { $ oty { handle : self . handle , _marker : PhantomData , } . drop () ; } } impl < S > Encode < S > for $ oty { fn encode (self , w : & mut Writer , s : & mut S) { mem :: ManuallyDrop :: new (self) . handle . encode (w , s) ; } } impl < S > Encode < S > for &$ oty { fn encode (self , w : & mut Writer , s : & mut S) { self . handle . encode (w , s) ; } } impl < S > Encode < S > for & mut $ oty { fn encode (self , w : & mut Writer , s : & mut S) { self . handle . encode (w , s) ; } } impl < S > DecodeMut <'_ , '_ , S > for $ oty { fn decode (r : & mut Reader <'_ >, s : & mut S) -> Self { $ oty { handle : handle :: Handle :: decode (r , s) , _marker : PhantomData , } } }) * $ (# [derive (Copy , Clone , PartialEq , Eq , Hash)] pub (crate) struct $ ity { handle : handle :: Handle , _marker : PhantomData <* mut () >, } impl < S > Encode < S > for $ ity { fn encode (self , w : & mut Writer , s : & mut S) { self . handle . encode (w , s) ; } } impl < S > DecodeMut <'_ , '_ , S > for $ ity { fn decode (r : & mut Reader <'_ >, s : & mut S) -> Self { $ ity { handle : handle :: Handle :: decode (r , s) , _marker : PhantomData , } } }) * } }}
mkitem!{with_api_handle_types ! (define_client_handles) ;}
mkitem!{mkimpl!{impl Clone for TokenStream { fn clone (& self) -> Self { self . clone () } }}}
mkitem!{mkimpl!{impl Span { pub (crate) fn def_site () -> Span { Bridge :: with (| bridge | bridge . globals . def_site) } pub (crate) fn call_site () -> Span { Bridge :: with (| bridge | bridge . globals . call_site) } pub (crate) fn mixed_site () -> Span { Bridge :: with (| bridge | bridge . globals . mixed_site) } }}}
mkitem!{mkimpl!{impl fmt :: Debug for Span { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (& self . debug ()) } }}}
mkuse!{pub (crate) use super :: symbol :: Symbol ;}
mkitem!{macro_rules ! define_client_side { ($ ($ name : ident { $ (fn $ method : ident ($ ($ arg : ident : $ arg_ty : ty) ,* $ (,) ?) $ (-> $ ret_ty : ty) ?;) * }) ,* $ (,) ?) => { $ (impl $ name { $ (pub (crate) fn $ method ($ ($ arg : $ arg_ty) ,*) $ (-> $ ret_ty) ? { Bridge :: with (| bridge | { let mut buf = bridge . cached_buffer . take () ; buf . clear () ; api_tags :: Method ::$ name (api_tags ::$ name ::$ method) . encode (& mut buf , & mut ()) ; reverse_encode ! (buf ; $ ($ arg) ,*) ; buf = bridge . dispatch . call (buf) ; let r = Result ::< _ , PanicMessage >:: decode (& mut & buf [..] , & mut ()) ; bridge . cached_buffer = buf ; r . unwrap_or_else (| e | panic :: resume_unwind (e . into ())) }) }) * }) * } }}
mkitem!{with_api ! (self , self , define_client_side) ;}
mkitem!{mkstruct!{struct Bridge < 'a > { # [doc = " Reusable buffer (only `clear`-ed, never shrunk), primarily"] # [doc = " used for making requests."] cached_buffer : Buffer , # [doc = " Server-side function that the client uses to make requests."] dispatch : closure :: Closure < 'a , Buffer , Buffer > , # [doc = " Provided globals for this macro expansion."] globals : ExpnGlobals < Span > , }}}
mkitem!{mkimpl!{impl < 'a > ! Send for Bridge < 'a > { }}}
mkitem!{mkimpl!{impl < 'a > ! Sync for Bridge < 'a > { }}}
mkmod!{state, { 
                getname!(state);
                getsrc!(state);
                getpath!(state);
                get_deps!(state);
                get_crates!(state);
                mkinclude!(state);
                mkuse!{use std :: cell :: { Cell , RefCell } ;}
mkuse!{use std :: ptr ;}
mkuse!{use super :: Bridge ;}
mkitem!{thread_local ! { static BRIDGE_STATE : Cell <* const () > = const { Cell :: new (ptr :: null ()) } ; }}

macro_rules! set_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set in module {}", module_path!());
    };
}

mkfn!{
    set_introspect!();
    pub (super) fn set < 'bridge , R > (state : & RefCell < Bridge < 'bridge > > , f : impl FnOnce () -> R) -> R { struct RestoreOnDrop (* const ()) ; impl Drop for RestoreOnDrop { fn drop (& mut self) { BRIDGE_STATE . set (self . 0) ; } } let inner = ptr :: from_ref (state) . cast () ; let outer = BRIDGE_STATE . replace (inner) ; let _restore = RestoreOnDrop (outer) ; f () }
}

macro_rules! with_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with in module {}", module_path!());
    };
}

mkfn!{
    with_introspect!();
    pub (super) fn with < R > (f : impl for < 'bridge > FnOnce (Option < & RefCell < Bridge < 'bridge > > >) -> R ,) -> R { let state = BRIDGE_STATE . get () ; let bridge = unsafe { state . cast :: < RefCell < Bridge < 'static > > > () . as_ref () } ; f (bridge) }
} 
            }}
mkitem!{mkimpl!{impl Bridge < '_ > { fn with < R > (f : impl FnOnce (& mut Bridge < '_ >) -> R) -> R { state :: with (| state | { let bridge = state . expect ("procedural macro API is used outside of a procedural macro") ; let mut bridge = bridge . try_borrow_mut () . expect ("procedural macro API is used while it's already in use") ; f (& mut bridge) }) } }}}

macro_rules! is_available_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_available in module {}", module_path!());
    };
}

mkfn!{
    is_available_introspect!();
    pub (crate) fn is_available () -> bool { state :: with (| s | s . is_some ()) }
}
mkitem!{mkstruct!{# [doc = " A client-side RPC entry-point, which may be using a different `proc_macro`"] # [doc = " from the one used by the server, but can be invoked compatibly."] # [doc = ""] # [doc = " Note that the (phantom) `I` (\"input\") and `O` (\"output\") type parameters"] # [doc = " decorate the `Client<I, O>` with the RPC \"interface\" of the entry-point, but"] # [doc = " do not themselves participate in ABI, at all, only facilitate type-checking."] # [doc = ""] # [doc = " E.g. `Client<TokenStream, TokenStream>` is the common proc macro interface,"] # [doc = " used for `#[proc_macro] fn foo(input: TokenStream) -> TokenStream`,"] # [doc = " indicating that the RPC input and output will be serialized token streams,"] # [doc = " and forcing the use of APIs that take/return `S::TokenStream`, server-side."] # [repr (C)] pub struct Client < I , O > { pub (super) handle_counters : & 'static HandleCounters , pub (super) run : extern "C" fn (BridgeConfig < '_ >) -> Buffer , pub (super) _marker : PhantomData < fn (I) -> O > , }}}
mkitem!{mkimpl!{impl < I , O > Copy for Client < I , O > { }}}
mkitem!{mkimpl!{impl < I , O > Clone for Client < I , O > { fn clone (& self) -> Self { * self } }}}

macro_rules! maybe_install_panic_hook_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_install_panic_hook in module {}", module_path!());
    };
}

mkfn!{
    maybe_install_panic_hook_introspect!();
    fn maybe_install_panic_hook (force_show_panics : bool) { static HIDE_PANICS_DURING_EXPANSION : Once = Once :: new () ; HIDE_PANICS_DURING_EXPANSION . call_once (| | { let prev = panic :: take_hook () ; panic :: set_hook (Box :: new (move | info | { if force_show_panics || ! is_available () || ! info . can_unwind () { prev (info) } })) ; }) ; }
}

macro_rules! run_client_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_client in module {}", module_path!());
    };
}

mkfn!{
    run_client_introspect!();
    # [doc = " Client-side helper for handling client panics, entering the bridge,"] # [doc = " deserializing input and serializing output."] fn run_client < A : for < 'a , 's > DecodeMut < 'a , 's , () > , R : Encode < () > > (config : BridgeConfig < '_ > , f : impl FnOnce (A) -> R ,) -> Buffer { let BridgeConfig { input : mut buf , dispatch , force_show_panics , .. } = config ; panic :: catch_unwind (panic :: AssertUnwindSafe (| | { maybe_install_panic_hook (force_show_panics) ; Symbol :: invalidate_all () ; let reader = & mut & buf [..] ; let (globals , input) = < (ExpnGlobals < Span > , A) > :: decode (reader , & mut ()) ; let state = RefCell :: new (Bridge { cached_buffer : buf . take () , dispatch , globals }) ; let output = state :: set (& state , | | f (input)) ; buf = RefCell :: into_inner (state) . cached_buffer ; buf . clear () ; Ok :: < _ , () > (output) . encode (& mut buf , & mut ()) ; })) . map_err (PanicMessage :: from) . unwrap_or_else (| e | { buf . clear () ; Err :: < () , _ > (e) . encode (& mut buf , & mut ()) ; }) ; Symbol :: invalidate_all () ; buf }
}
mkitem!{mkimpl!{impl Client < crate :: TokenStream , crate :: TokenStream > { pub const fn expand1 (f : impl Fn (crate :: TokenStream) -> crate :: TokenStream + Copy) -> Self { Client { handle_counters : & COUNTERS , run : super :: selfless_reify :: reify_to_extern_c_fn_hrt_bridge (move | bridge | { run_client (bridge , | input | f (crate :: TokenStream (Some (input))) . 0) }) , _marker : PhantomData , } } }}}
mkitem!{mkimpl!{impl Client < (crate :: TokenStream , crate :: TokenStream) , crate :: TokenStream > { pub const fn expand2 (f : impl Fn (crate :: TokenStream , crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { Client { handle_counters : & COUNTERS , run : super :: selfless_reify :: reify_to_extern_c_fn_hrt_bridge (move | bridge | { run_client (bridge , | (input , input2) | { f (crate :: TokenStream (Some (input)) , crate :: TokenStream (Some (input2))) . 0 }) }) , _marker : PhantomData , } } }}}
mkitem!{mkenum!{# [repr (C)] # [derive (Copy , Clone)] pub enum ProcMacro { CustomDerive { trait_name : & 'static str , attributes : & 'static [& 'static str] , client : Client < crate :: TokenStream , crate :: TokenStream > , } , Attr { name : & 'static str , client : Client < (crate :: TokenStream , crate :: TokenStream) , crate :: TokenStream > , } , Bang { name : & 'static str , client : Client < crate :: TokenStream , crate :: TokenStream > , } , }}}
mkitem!{mkimpl!{impl ProcMacro { pub fn name (& self) -> & 'static str { match self { ProcMacro :: CustomDerive { trait_name , .. } => trait_name , ProcMacro :: Attr { name , .. } => name , ProcMacro :: Bang { name , .. } => name , } } pub const fn custom_derive (trait_name : & 'static str , attributes : & 'static [& 'static str] , expand : impl Fn (crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { ProcMacro :: CustomDerive { trait_name , attributes , client : Client :: expand1 (expand) } } pub const fn attr (name : & 'static str , expand : impl Fn (crate :: TokenStream , crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { ProcMacro :: Attr { name , client : Client :: expand2 (expand) } } pub const fn bang (name : & 'static str , expand : impl Fn (crate :: TokenStream) -> crate :: TokenStream + Copy ,) -> Self { ProcMacro :: Bang { name , client : Client :: expand1 (expand) } } }}}