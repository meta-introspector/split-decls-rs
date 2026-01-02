mkuse!{use smallvec :: { SmallVec , smallvec } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use crate :: data_structures :: SsoHashSet ;}
mkuse!{use crate :: inherent :: * ;}
mkuse!{use crate :: { self as ty , Interner } ;}
mkitem!{type TypeWalkerStack < I > = SmallVec < [< I as Interner > :: GenericArg ; 8] > ;}
mkitem!{mkstruct!{# [doc = " An iterator for walking the type tree."] # [doc = ""] # [doc = " It's very easy to produce a deeply"] # [doc = " nested type tree with a lot of"] # [doc = " identical subtrees. In order to work efficiently"] # [doc = " in this situation walker only visits each type once."] # [doc = " It maintains a set of visited types and"] # [doc = " skips any types that are already there."] pub struct TypeWalker < I : Interner > { stack : TypeWalkerStack < I > , last_subtree : usize , pub visited : SsoHashSet < I :: GenericArg > , }}}
mkitem!{mkimpl!{impl < I : Interner > TypeWalker < I > { pub fn new (root : I :: GenericArg) -> Self { Self { stack : smallvec ! [root] , last_subtree : 1 , visited : SsoHashSet :: new () } } # [doc = " Skips the subtree corresponding to the last type"] # [doc = " returned by `next()`."] # [doc = ""] # [doc = " Example: Imagine you are walking `Foo<Bar<i32>, usize>`."] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " let mut iter: TypeWalker = ...;"] # [doc = " iter.next(); // yields Foo"] # [doc = " iter.next(); // yields Bar<i32>"] # [doc = " iter.skip_current_subtree(); // skips i32"] # [doc = " iter.next(); // yields usize"] # [doc = " ```"] pub fn skip_current_subtree (& mut self) { self . stack . truncate (self . last_subtree) ; } }}}
mkitem!{mkimpl!{impl < I : Interner > Iterator for TypeWalker < I > { type Item = I :: GenericArg ; fn next (& mut self) -> Option < I :: GenericArg > { debug ! ("next(): stack={:?}" , self . stack) ; loop { let next = self . stack . pop () ? ; self . last_subtree = self . stack . len () ; if self . visited . insert (next) { push_inner :: < I > (& mut self . stack , next) ; debug ! ("next: stack={:?}" , self . stack) ; return Some (next) ; } } } }}}

macro_rules! push_inner_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function push_inner in module {}", module_path!());
    };
}

mkfn!{
    push_inner_introspect!();
    # [doc = " We push `GenericArg`s on the stack in reverse order so as to"] # [doc = " maintain a pre-order traversal. As of the time of this"] # [doc = " writing, the fact that the traversal is pre-order is not"] # [doc = " known to be significant to any code, but it seems like the"] # [doc = " natural order one would expect (basically, the order of the"] # [doc = " types as they are written)."] fn push_inner < I : Interner > (stack : & mut TypeWalkerStack < I > , parent : I :: GenericArg) { match parent . kind () { ty :: GenericArgKind :: Type (parent_ty) => match parent_ty . kind () { ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str | ty :: Infer (_) | ty :: Param (_) | ty :: Never | ty :: Error (_) | ty :: Placeholder (..) | ty :: Bound (..) | ty :: Foreign (..) => { } ty :: Pat (ty , pat) => { push_ty_pat :: < I > (stack , pat) ; stack . push (ty . into ()) ; } ty :: Array (ty , len) => { stack . push (len . into ()) ; stack . push (ty . into ()) ; } ty :: Slice (ty) => { stack . push (ty . into ()) ; } ty :: RawPtr (ty , _) => { stack . push (ty . into ()) ; } ty :: Ref (lt , ty , _) => { stack . push (ty . into ()) ; stack . push (lt . into ()) ; } ty :: Alias (_ , data) => { stack . extend (data . args . iter () . rev ()) ; } ty :: Dynamic (obj , lt , _) => { stack . push (lt . into ()) ; stack . extend (obj . iter () . rev () . filter_map (| predicate | { let (args , opt_ty) = match predicate . skip_binder () { ty :: ExistentialPredicate :: Trait (tr) => (tr . args , None) , ty :: ExistentialPredicate :: Projection (p) => (p . args , Some (p . term)) , ty :: ExistentialPredicate :: AutoTrait (_) => { return None ; } } ; Some (args . iter () . rev () . chain (opt_ty . map (| term | match term . kind () { ty :: TermKind :: Ty (ty) => ty . into () , ty :: TermKind :: Const (ct) => ct . into () , }))) }) . flatten () ,) ; } ty :: Adt (_ , args) | ty :: Closure (_ , args) | ty :: CoroutineClosure (_ , args) | ty :: Coroutine (_ , args) | ty :: CoroutineWitness (_ , args) | ty :: FnDef (_ , args) => { stack . extend (args . iter () . rev ()) ; } ty :: Tuple (ts) => stack . extend (ts . iter () . rev () . map (| ty | ty . into ())) , ty :: FnPtr (sig_tys , _hdr) => { stack . extend (sig_tys . skip_binder () . inputs_and_output . iter () . rev () . map (| ty | ty . into ()) ,) ; } ty :: UnsafeBinder (bound_ty) => { stack . push (bound_ty . skip_binder () . into ()) ; } } , ty :: GenericArgKind :: Lifetime (_) => { } ty :: GenericArgKind :: Const (parent_ct) => match parent_ct . kind () { ty :: ConstKind :: Infer (_) | ty :: ConstKind :: Param (_) | ty :: ConstKind :: Placeholder (_) | ty :: ConstKind :: Bound (..) | ty :: ConstKind :: Error (_) => { } ty :: ConstKind :: Value (cv) => stack . push (cv . ty () . into ()) , ty :: ConstKind :: Expr (expr) => stack . extend (expr . args () . iter () . rev ()) , ty :: ConstKind :: Unevaluated (ct) => { stack . extend (ct . args . iter () . rev ()) ; } } , } }
}

macro_rules! push_ty_pat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function push_ty_pat in module {}", module_path!());
    };
}

mkfn!{
    push_ty_pat_introspect!();
    fn push_ty_pat < I : Interner > (stack : & mut TypeWalkerStack < I > , pat : I :: Pat) { match pat . kind () { ty :: PatternKind :: Range { start , end } => { stack . push (end . into ()) ; stack . push (start . into ()) ; } ty :: PatternKind :: Or (pats) => { for pat in pats . iter () { push_ty_pat :: < I > (stack , pat) } } } }
}