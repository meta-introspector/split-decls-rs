macro_rules! deps {
    () => {
        GenericBounds!();
        Impl!();
        Pat!();
        FnPtrTy!();
        Lifetime!();
        Walkable!();
        BoundKind!();
        Ty!();
        TyPat!();
        QSelf!();
        TraitObjectSyntax!();
        Path!();
        UnsafeBinderTy!();
        MutTy!();
        MacCall!();
        LifetimeCtxt!();
        AnonConst!();
        Type!();
        Trait!();
    };
}

macro_rules! TyKind {
    () => {
        deps!();
        # [doc = " The various kinds of type recognized by the compiler."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum TyKind { # [doc = " A variable-length slice (`[T]`)."] Slice (Box < Ty >) , # [doc = " A fixed length array (`[T; n]`)."] Array (Box < Ty > , AnonConst) , # [doc = " A raw pointer (`*const T` or `*mut T`)."] Ptr (MutTy) , # [doc = " A reference (`&'a T` or `&'a mut T`)."] Ref (# [visitable (extra = LifetimeCtxt :: Ref)] Option < Lifetime > , MutTy) , # [doc = " A pinned reference (`&'a pin const T` or `&'a pin mut T`)."] # [doc = ""] # [doc = " Desugars into `Pin<&'a T>` or `Pin<&'a mut T>`."] PinnedRef (# [visitable (extra = LifetimeCtxt :: Ref)] Option < Lifetime > , MutTy) , # [doc = " A function pointer type (e.g., `fn(usize) -> bool`)."] FnPtr (Box < FnPtrTy >) , # [doc = " An unsafe existential lifetime binder (e.g., `unsafe<'a> &'a ()`)."] UnsafeBinder (Box < UnsafeBinderTy >) , # [doc = " The never type (`!`)."] Never , # [doc = " A tuple (`(A, B, C, D,...)`)."] Tup (ThinVec < Box < Ty > >) , # [doc = " A path (`module::module::...::Type`), optionally"] # [doc = " \"qualified\", e.g., `<Vec<T> as SomeTrait>::SomeType`."] # [doc = ""] # [doc = " Type parameters are stored in the `Path` itself."] Path (Option < Box < QSelf > > , Path) , # [doc = " A trait object type `Bound1 + Bound2 + Bound3`"] # [doc = " where `Bound` is a trait or a lifetime."] TraitObject (# [visitable (extra = BoundKind :: TraitObject)] GenericBounds , TraitObjectSyntax) , # [doc = " An `impl Bound1 + Bound2 + Bound3` type"] # [doc = " where `Bound` is a trait or a lifetime."] # [doc = ""] # [doc = " The `NodeId` exists to prevent lowering from having to"] # [doc = " generate `NodeId`s on the fly, which would complicate"] # [doc = " the generation of opaque `type Foo = impl Trait` items significantly."] ImplTrait (NodeId , # [visitable (extra = BoundKind :: Impl)] GenericBounds) , # [doc = " No-op; kept solely so that we can pretty-print faithfully."] Paren (Box < Ty >) , # [doc = " Unused for now."] Typeof (AnonConst) , # [doc = " This means the type should be inferred instead of it having been"] # [doc = " specified. This can appear anywhere in a type."] Infer , # [doc = " Inferred type of a `self` or `&self` argument in a method."] ImplicitSelf , # [doc = " A macro in the type position."] MacCall (Box < MacCall >) , # [doc = " Placeholder for a `va_list`."] CVarArgs , # [doc = " Pattern types like `pattern_type!(u32 is 1..=)`, which is the same as `NonZero<u32>`,"] # [doc = " just as part of the type system."] Pat (Box < Ty > , Box < TyPat >) , # [doc = " Sometimes we need a dummy value when no error has occurred."] Dummy , # [doc = " Placeholder for a kind that has failed to be defined."] Err (ErrorGuaranteed) , }
    };
}

TyKind!()