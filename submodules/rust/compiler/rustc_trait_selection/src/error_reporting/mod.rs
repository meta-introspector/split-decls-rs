mkuse!{use std :: ops :: Deref ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_infer :: infer :: InferCtxt ;}
mkuse!{use rustc_infer :: traits :: PredicateObligations ;}
mkuse!{use rustc_macros :: extension ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkmod!{infer, { 
                getname!(infer);
                getsrc!(infer);
                getpath!(infer);
                get_deps!(infer);
                get_crates!(infer);
                mkinclude!(infer);
                 
            }}
mkmod!{traits, { 
                getname!(traits);
                getsrc!(traits);
                getpath!(traits);
                get_deps!(traits);
                get_crates!(traits);
                mkinclude!(traits);
                 
            }}
mkitem!{mkstruct!{# [doc = " A helper for building type related errors. The `typeck_results`"] # [doc = " field is only populated during an in-progress typeck."] # [doc = " Get an instance by calling `InferCtxt::err_ctxt` or `FnCtxt::err_ctxt`."] # [doc = ""] # [doc = " You must only create this if you intend to actually emit an error (or"] # [doc = " perhaps a warning, though preferably not.) It provides a lot of utility"] # [doc = " methods which should not be used during the happy path."] pub struct TypeErrCtxt < 'a , 'tcx > { pub infcx : & 'a InferCtxt < 'tcx > , pub typeck_results : Option < std :: cell :: Ref < 'a , ty :: TypeckResults < 'tcx > > > , pub fallback_has_occurred : bool , pub normalize_fn_sig : Box < dyn Fn (ty :: PolyFnSig < 'tcx >) -> ty :: PolyFnSig < 'tcx > + 'a > , pub autoderef_steps : Box < dyn Fn (Ty < 'tcx >) -> Vec < (Ty < 'tcx > , PredicateObligations < 'tcx >) > + 'a > , }}}
mkitem!{mkimpl!{# [extension (pub trait InferCtxtErrorExt <'tcx >)] impl < 'tcx > InferCtxt < 'tcx > { # [doc = " Creates a `TypeErrCtxt` for emitting various inference errors."] # [doc = " During typeck, use `FnCtxt::err_ctxt` instead."] fn err_ctxt (& self) -> TypeErrCtxt < '_ , 'tcx > { TypeErrCtxt { infcx : self , typeck_results : None , fallback_has_occurred : false , normalize_fn_sig : Box :: new (| fn_sig | fn_sig) , autoderef_steps : Box :: new (| ty | { debug_assert ! (false , "shouldn't be using autoderef_steps outside of typeck") ; vec ! [(ty , PredicateObligations :: new ())] }) , } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > TypeErrCtxt < 'a , 'tcx > { pub fn dcx (& self) -> DiagCtxtHandle < 'a > { self . infcx . dcx () } # [doc = " This is just to avoid a potential footgun of accidentally"] # [doc = " dropping `typeck_results` by calling `InferCtxt::err_ctxt`"] # [deprecated (note = "you already have a `TypeErrCtxt`")] # [allow (unused)] pub fn err_ctxt (& self) -> ! { bug ! ("called `err_ctxt` on `TypeErrCtxt`. Try removing the call") ; } }}}
mkitem!{mkimpl!{impl < 'tcx > Deref for TypeErrCtxt < '_ , 'tcx > { type Target = InferCtxt < 'tcx > ; fn deref (& self) -> & InferCtxt < 'tcx > { self . infcx } }}}