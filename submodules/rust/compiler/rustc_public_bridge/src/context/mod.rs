mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use rustc_abi :: HasDataLayout ;}
mkuse!{use rustc_middle :: ty ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiOfHelpers , HasTyCtxt , HasTypingEnv , LayoutOfHelpers } ;}
mkuse!{use rustc_middle :: ty :: { Ty , TyCtxt } ;}
mkuse!{use crate :: { Bridge , Error } ;}
mkmod!{helpers, { 
                getname!(helpers);
                getsrc!(helpers);
                getpath!(helpers);
                get_deps!(helpers);
                get_crates!(helpers);
                mkinclude!(helpers);
                 
            }}
mkmod!{impls, { 
                getname!(impls);
                getsrc!(impls);
                getpath!(impls);
                get_deps!(impls);
                get_crates!(impls);
                mkinclude!(impls);
                 
            }}
mkuse!{pub use helpers :: * ;}
mkitem!{mkstruct!{# [doc = " Provides direct access to rustc's internal queries."] # [doc = ""] # [doc = " `CompilerInterface` must go through"] # [doc = " this context to obtain internal information."] pub struct CompilerCtxt < 'tcx , B : Bridge > { pub tcx : TyCtxt < 'tcx > , _marker : PhantomData < B > , }}}
mkitem!{mkimpl!{impl < 'tcx , B : Bridge > CompilerCtxt < 'tcx , B > { pub fn new (tcx : TyCtxt < 'tcx >) -> Self { Self { tcx , _marker : Default :: default () } } }}}
mkitem!{mkimpl!{# [doc = " Implement error handling for extracting function ABI information."] impl < 'tcx , B : Bridge > FnAbiOfHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { type FnAbiOfResult = Result < & 'tcx rustc_target :: callconv :: FnAbi < 'tcx , Ty < 'tcx > > , B :: Error > ; # [inline] fn handle_fn_abi_err (& self , err : ty :: layout :: FnAbiError < 'tcx > , _span : rustc_span :: Span , fn_abi_request : ty :: layout :: FnAbiRequest < 'tcx > ,) -> B :: Error { B :: Error :: new (format ! ("Failed to get ABI for `{fn_abi_request:?}`: {err:?}")) } }}}
mkitem!{mkimpl!{impl < 'tcx , B : Bridge > LayoutOfHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { type LayoutOfResult = Result < ty :: layout :: TyAndLayout < 'tcx > , B :: Error > ; # [inline] fn handle_layout_err (& self , err : ty :: layout :: LayoutError < 'tcx > , _span : rustc_span :: Span , ty : Ty < 'tcx > ,) -> B :: Error { B :: Error :: new (format ! ("Failed to get layout for `{ty}`: {err}")) } }}}
mkitem!{mkimpl!{impl < 'tcx , B : Bridge > HasTypingEnv < 'tcx > for CompilerCtxt < 'tcx , B > { fn typing_env (& self) -> ty :: TypingEnv < 'tcx > { ty :: TypingEnv :: fully_monomorphized () } }}}
mkitem!{mkimpl!{impl < 'tcx , B : Bridge > HasTyCtxt < 'tcx > for CompilerCtxt < 'tcx , B > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }}}
mkitem!{mkimpl!{impl < 'tcx , B : Bridge > HasDataLayout for CompilerCtxt < 'tcx , B > { fn data_layout (& self) -> & rustc_abi :: TargetDataLayout { self . tcx . data_layout () } }}}