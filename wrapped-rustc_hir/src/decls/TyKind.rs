macro_rules! deps {
    () => {
        TyPat!();
        MutTy!();
        QPath!();
        Lifetime!();
        FnPtrTy!();
        PathSegment!();
        InferDelegationKind!();
        ConstArg!();
        PolyTraitRef!();
        UnsafeBinderTy!();
        Pat!();
        Path!();
        AnonConst!();
        GenericBounds!();
        OpaqueTy!();
        Ty!();
    };
}

macro_rules! TyKind {
    () => {
        deps!();
        # [doc = " The various kinds of types recognized by the compiler."] # [doc = ""] # [doc = " For an explanation of the `Unambig` generic parameter see the dev-guide:"] # [doc = " <https://rustc-dev-guide.rust-lang.org/hir/ambig-unambig-ty-and-consts.html>"] # [repr (u8 , C)] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum TyKind < 'hir , Unambig = () > { # [doc = " Actual type should be inherited from `DefId` signature"] InferDelegation (DefId , InferDelegationKind) , # [doc = " A variable length slice (i.e., `[T]`)."] Slice (& 'hir Ty < 'hir >) , # [doc = " A fixed length array (i.e., `[T; n]`)."] Array (& 'hir Ty < 'hir > , & 'hir ConstArg < 'hir >) , # [doc = " A raw pointer (i.e., `*const T` or `*mut T`)."] Ptr (MutTy < 'hir >) , # [doc = " A reference (i.e., `&'a T` or `&'a mut T`)."] Ref (& 'hir Lifetime , MutTy < 'hir >) , # [doc = " A function pointer (e.g., `fn(usize) -> bool`)."] FnPtr (& 'hir FnPtrTy < 'hir >) , # [doc = " An unsafe binder type (e.g. `unsafe<'a> Foo<'a>`)."] UnsafeBinder (& 'hir UnsafeBinderTy < 'hir >) , # [doc = " The never type (`!`)."] Never , # [doc = " A tuple (`(A, B, C, D, ...)`)."] Tup (& 'hir [Ty < 'hir >]) , # [doc = " A path to a type definition (`module::module::...::Type`), or an"] # [doc = " associated type (e.g., `<Vec<T> as Trait>::Type` or `<T>::Target`)."] # [doc = ""] # [doc = " Type parameters may be stored in each `PathSegment`."] Path (QPath < 'hir >) , # [doc = " An opaque type definition itself. This is only used for `impl Trait`."] OpaqueDef (& 'hir OpaqueTy < 'hir >) , # [doc = " A trait ascription type, which is `impl Trait` within a local binding."] TraitAscription (GenericBounds < 'hir >) , # [doc = " A trait object type `Bound1 + Bound2 + Bound3`"] # [doc = " where `Bound` is a trait or a lifetime."] # [doc = ""] # [doc = " We use pointer tagging to represent a `&'hir Lifetime` and `TraitObjectSyntax` pair"] # [doc = " as otherwise this type being `repr(C)` would result in `TyKind` increasing in size."] TraitObject (& 'hir [PolyTraitRef < 'hir >] , TaggedRef < 'hir , Lifetime , TraitObjectSyntax >) , # [doc = " Unused for now."] Typeof (& 'hir AnonConst) , # [doc = " Placeholder for a type that has failed to be defined."] Err (rustc_span :: ErrorGuaranteed) , # [doc = " Pattern types (`pattern_type!(u32 is 1..)`)"] Pat (& 'hir Ty < 'hir > , & 'hir TyPat < 'hir >) , # [doc = " `TyKind::Infer` means the type should be inferred instead of it having been"] # [doc = " specified. This can appear anywhere in a type."] # [doc = ""] # [doc = " This variant is not always used to represent inference types, sometimes"] # [doc = " [`GenericArg::Infer`] is used instead."] Infer (Unambig) , }
    };
}

TyKind!();