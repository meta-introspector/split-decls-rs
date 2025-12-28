macro_rules! deps {
    () => {
        DefiningTy!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl < 'tcx > DefiningTy < 'tcx > { # [doc = " Returns a list of all the upvar types for this MIR. If this is"] # [doc = " not a closure or coroutine, there are no upvars, and hence it"] # [doc = " will be an empty list. The order of types in this list will"] # [doc = " match up with the upvar order in the HIR, typesystem, and MIR."] pub (crate) fn upvar_tys (self) -> & 'tcx ty :: List < Ty < 'tcx > > { match self { DefiningTy :: Closure (_ , args) => args . as_closure () . upvar_tys () , DefiningTy :: CoroutineClosure (_ , args) => args . as_coroutine_closure () . upvar_tys () , DefiningTy :: Coroutine (_ , args) => args . as_coroutine () . upvar_tys () , DefiningTy :: FnDef (..) | DefiningTy :: Const (..) | DefiningTy :: InlineConst (..) | DefiningTy :: GlobalAsm (_) => ty :: List :: empty () , } } # [doc = " Number of implicit inputs -- notably the \"environment\""] # [doc = " parameter for closures -- that appear in MIR but not in the"] # [doc = " user's code."] pub (crate) fn implicit_inputs (self) -> usize { match self { DefiningTy :: Closure (..) | DefiningTy :: CoroutineClosure (..) | DefiningTy :: Coroutine (..) => 1 , DefiningTy :: FnDef (..) | DefiningTy :: Const (..) | DefiningTy :: InlineConst (..) | DefiningTy :: GlobalAsm (_) => 0 , } } pub (crate) fn is_fn_def (& self) -> bool { matches ! (* self , DefiningTy :: FnDef (..)) } pub (crate) fn is_const (& self) -> bool { matches ! (* self , DefiningTy :: Const (..) | DefiningTy :: InlineConst (..)) } pub (crate) fn def_id (& self) -> DefId { match * self { DefiningTy :: Closure (def_id , ..) | DefiningTy :: CoroutineClosure (def_id , ..) | DefiningTy :: Coroutine (def_id , ..) | DefiningTy :: FnDef (def_id , ..) | DefiningTy :: Const (def_id , ..) | DefiningTy :: InlineConst (def_id , ..) | DefiningTy :: GlobalAsm (def_id) => def_id , } } # [doc = " Returns the args of the `DefiningTy`. These are equivalent to the identity"] # [doc = " substs of the body, but replaced with region vids."] pub (crate) fn args (& self) -> ty :: GenericArgsRef < 'tcx > { match * self { DefiningTy :: Closure (_ , args) | DefiningTy :: Coroutine (_ , args) | DefiningTy :: CoroutineClosure (_ , args) | DefiningTy :: FnDef (_ , args) | DefiningTy :: Const (_ , args) | DefiningTy :: InlineConst (_ , args) => args , DefiningTy :: GlobalAsm (_) => ty :: List :: empty () , } } }
    };
}

impl_491!()