macro_rules! FnDeclKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum FnDeclKind { Fn , Inherent , ExternFn , Closure , Pointer , Trait , Impl , }
    };
}

FnDeclKind!()